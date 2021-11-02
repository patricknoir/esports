use core::actix_web;
use actix_web::web::{Json, Data};
use crate::model::user::{NewUser, User, UserChange};
use crate::{AppState, JWT_SECRET, HASH_SECRET, util};
use core::actix_web::{Responder, HttpRequest};
use core::api::AppResponse;
use core::error::AppError;
use validator::Validate;
use core::jwt::{CanGenerateJwt, CanDecodeJwt};
use crate::model::claims::Claims;
use crate::model::credentials::Credentials;
use core::jsonwebtoken::TokenData;
use crate::api::{GetById, UpdateUser, GetByEmail};
use core::prelude::*;
use crate::model::password_reset_request::{PasswordResetRequest, NewPassword};
use crate::util::Config;
use chrono::Utc;
use crate::model::password_reset_claims::PasswordResetClaims;
use crate::io::refresh_password_publisher::RefreshPasswordMessage;
use serde::Deserialize;

fn hash_password(email: &str, password: &str) -> Result<String> {
	core::crypto::hash_password( email,password, std::env::var(HASH_SECRET).unwrap().as_str())
}

#[actix_web::post("/users")]
pub async fn register(new_user: Json<NewUser>, state: Data<AppState>) -> impl Responder {
	if let Err(validation_errors) = new_user.validate() {
		let app_error: AppError = validation_errors.into();
		Err(app_error)
	} else {
		let hashed_password: String = hash_password(new_user.email.as_str(), new_user.password.as_str())?;
		let form = NewUser {
			password: hashed_password,
			..new_user.0
		};
		state.data_service.send(form).await.unwrap()
			.map(|user| {
				let claims: Claims = user.clone().into();
				AppResponse::<User>::created(claims.to_jwt(std::env::var(JWT_SECRET).unwrap()).ok(), Some(user))
			})
			.map_err(|err| err.into())
	}
}

#[actix_web::post("/login")]
pub async fn login(credentials: Json<Credentials>, state: Data<AppState>) -> impl Responder {
	if let Err(validation_errors) = credentials.validate() {
		let app_error: AppError = validation_errors.into();
		Err(app_error)
	} else {
		let hashed_password: String = hash_password(credentials.0.email.as_str(), credentials.0.password.as_str())?;
		let form = Credentials {
			password: hashed_password,
			..credentials.0
		};
		match state.data_service.send(form).await.unwrap() {
			Ok(user) => {
				let claims: Claims = user.clone().into();
				claims.to_jwt(std::env::var(JWT_SECRET).unwrap()).map(|token| AppResponse::<User>::response(token, user))
					.map_err(|e| e.into())
			},
			Err(e) => Err(e.into())
		}
	}
}

#[actix_web::get("/users/me")]
pub async fn get_by_id(request: HttpRequest, state: Data<AppState>, config: Data<util::Config>) -> impl Responder {
	let jwt: TokenData<Claims> = request.decode_jwt(config.jwt_secret.clone())?;
	let request = GetById(jwt.claims.user_id);
	state.data_service.send(request).await.unwrap()
		.map(|user| AppResponse::<User>::body(user))
}

#[actix_web::put("/users/me")]
pub async fn update(changes: Json<UserChange>, request: HttpRequest, state: Data<AppState>, config: Data<util::Config>) -> impl Responder {
	let jwt: TokenData<Claims> = request.decode_jwt(config.jwt_secret.clone())?;
	let update_cmd = UpdateUser::new(jwt.claims.user_id, changes.0);
	state.data_service.send(update_cmd).await.unwrap()
		.map(|user| AppResponse::body(user))
}

#[actix_web::put("/users/password/reset")]
pub async fn request_password_reset(user_email: Json<PasswordResetRequest>, state: Data<AppState>, config: Data<Config>) -> impl Responder {
	if let Err(validation_errors) = user_email.validate() {
		let app_error: AppError = validation_errors.into();
		Err(app_error)
	} else {
		let user_result: Result<User> = state.data_service.send(GetByEmail(user_email.email.clone())).await.unwrap();

		if user_result.is_ok() {
			let user: User = user_result?;
			let exp: i64 = (Utc::now() + config.refresh_password_duration).timestamp();
			let jwt: String = PasswordResetClaims::new(user.id.clone(), user.email.clone(), exp)
				.to_jwt(config.hash_secret.clone())?;
			//send a command to the email service
			state.refresh_password_publisher.send(RefreshPasswordMessage {
				user_id: user.id,
				email: user.email.clone(),
				token: jwt.clone()
			}).await.expect("Error sending message to kafka")?;
		};

		let res: AppResponse<()> = AppResponse::empty();
		Ok(res)
	}
}

#[derive(Deserialize, Debug, Validate)]
pub struct CompletePasswordReset {
	pub token: String,
	#[validate(length(min = 1, message = "Password cannot be empty"))]
	#[serde(rename = "newPassword")]
	pub new_password: String
}

#[actix_web::put("/users/password/change")]
pub async fn complete_password_change(new_password: Json<NewPassword>, request: HttpRequest, config: Data<util::Config>, state: Data<AppState>) -> impl Responder {
	let jwt: TokenData<PasswordResetClaims> = request.decode_jwt(config.hash_secret.clone())?;
	if let Err(validation_errors) = new_password.validate() {
		let app_error: AppError = validation_errors.into();
		Err(app_error)
	} else {
		let change_password = UserChange {
			profile_picture: Option::None,
			username: Option::None,
			phone: Option::None,
			password: Option::None
		};
		let update_cmd = UpdateUser::new(jwt.claims.user_id, change_password);
		let user: User = state.data_service.send(update_cmd).await.unwrap()?;
		let claims: Claims = user.clone().into();
		claims.to_jwt(config.jwt_secret.clone()).map(|token| AppResponse::<User>::response(token, user))
			.map_err(|e| e.into())
	}
}

// #[actix_web::get("/users/password/change")]
// pub async fn complete_password_change(query: Query<CompletePasswordReset>, config: Data<util::Config>, state: Data<AppState>) -> impl Responder {
// 	let jwt: TokenData<PasswordResetClaims> = query.token.decode_jwt(config.hash_secret.clone())?;
// 	if let Err(validation_errors) = query.0.validate() {
// 		let app_error: AppError = validation_errors.into();
// 		Err(app_error)
// 	} else {
// 		let hashed_password = hash_password(jwt.claims.email.as_str(), query.new_password.as_str())?;
// 		let change_password = UserChange {
// 			profile_picture: Option::None,
// 			username: Option::None,
// 			phone: Option::None,
// 			password: Some(hashed_password)
// 		};
// 		let update_cmd = UpdateUser::new(jwt.claims.user_id, change_password);
// 		let user: User = state.data_service.send(update_cmd).await.unwrap()?;
// 		let claims: Claims = user.clone().into();
// 		claims.to_jwt(config.jwt_secret.clone()).map(|token| AppResponse::<User>::response(token, user))
// 			.map_err(|e| e.into())
// 	}
// }