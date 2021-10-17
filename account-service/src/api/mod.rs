mod health_check;
mod user;
use serde_json::json;
use core::actix_web::web::ServiceConfig;
use core::actix_web::HttpRequest;
use core::error::AppError;
use core::prelude::*;
use core::jwt::CanDecodeJwt;
use core::jsonwebtoken::TokenData;
use crate::model::claims::Claims;
use uuid::Uuid;
use core::actix::Message;
use crate::model::user::{User, UserChange};

const TOKEN_PREFIX: &str = "Bearer";

pub fn routes(cfg: &mut ServiceConfig) {
	cfg
		.service(health_check::health_check)
		.service(user::register)
		.service(user::login)
		.service(user::get_by_id)
		.service(user::update);
}

pub fn extract_authorization_header(req: &HttpRequest) -> Result<String> {
	let token = match req.headers().get(core::actix_web::http::header::AUTHORIZATION) {
		Some(token) => token.to_str().unwrap(),
		None => {
			return Err(AppError::Unauthorized(json!({
                "error": "No authorization was provided",
            })))
		}
	};

	if !token.starts_with(TOKEN_PREFIX) {
		return Err(AppError::Unauthorized(json!({
            "error": "Invalid authorization method",
        })));
	}

	let token = token.replacen(TOKEN_PREFIX, "", 1);
	let result = String::from(token.trim());
	Ok(result)
}

impl CanDecodeJwt<Claims> for HttpRequest {
	fn decode_jwt(&self, secret: String) -> Result<TokenData<Claims>> {
		let header = extract_authorization_header(self)?;
		header.decode_jwt(secret)
	}
}

pub struct GetById(pub Uuid);
impl Message for GetById {
	type Result = Result<User>;
}

#[derive(Debug)]
pub struct UpdateUser {
	pub id: Uuid,
	pub changes: UserChange
}

impl UpdateUser {
	pub fn new(id: Uuid, changes: UserChange) -> Self {
		UpdateUser {
			id,
			changes
		}
	}
}

impl Message for UpdateUser {
	type Result = Result<User>;
}