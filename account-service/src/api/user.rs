use core::actix_web;
use actix_web::web::{Json, Data};
use crate::model::user::{NewUser, User, UserChange};
use crate::{AppState, JWT_SECRET};
use core::actix_web::{Responder, HttpRequest};
use core::api::AppResponse;
use core::error::AppError;
use validator::Validate;
use core::jwt::{CanGenerateJwt, CanDecodeJwt};
use crate::model::claims::Claims;
use crate::model::credentials::Credentials;
use core::jsonwebtoken::TokenData;
use crate::api::{GetById, UpdateUser};

#[actix_web::post("/users")]
pub async fn register(new_user: Json<NewUser>, state: Data<AppState>) -> impl Responder {
	if let Err(validation_errors) = new_user.validate() {
		let app_error: AppError = validation_errors.into();
		Err(app_error)
	} else {
		let form = new_user.0;
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
		let form = credentials.0;
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
pub async fn get_by_id(request: HttpRequest, state: Data<AppState>) -> impl Responder {
	let jwt: TokenData<Claims> = request.decode_jwt(std::env::var(JWT_SECRET).unwrap())?;
	let request = GetById(jwt.claims.user_id);
	state.data_service.send(request).await.unwrap()
		.map(|user| AppResponse::<User>::body(user))
}

#[actix_web::put("/users/me")]
pub async fn update(changes: Json<UserChange>, request: HttpRequest, state: Data<AppState>) -> impl Responder {
	let jwt: TokenData<Claims> = request.decode_jwt(std::env::var(JWT_SECRET).unwrap())?;
	let update_cmd = UpdateUser::new(jwt.claims.user_id, changes.0);
	state.data_service.send(update_cmd).await.unwrap()
		.map(|user| AppResponse::body(user))
}