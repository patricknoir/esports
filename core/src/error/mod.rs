use serde_json::{json, Value as JsonValue};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};
use validator::ValidationErrors;
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use diesel::r2d2::PoolError;
use diesel::result::DatabaseErrorKind;
use diesel::result::{Error as DieselError};


#[derive(Debug, Fail)]
pub enum AppError {
	#[fail(display = "Internal Server Error")]
	InternalServerError,
	#[fail(display = "Not Found: {}", _0)]
	NotFound(JsonValue),
	#[fail(display = "Unauthorized: {}", _0)]
	Unauthorized(JsonValue),
	#[fail(display = "Bad Request: {}", _0)]
	BadRequest(JsonValue),
	#[fail(display = "Unprocessable Entity: {}", _0)]
	UnprocessableEntity(JsonValue),
}

impl AppError {
	pub fn internal_server_error() -> Self {
		AppError::InternalServerError
	}
	pub fn not_found(value: JsonValue) -> Self {
		AppError::NotFound(value)
	}
	pub fn bad_request(value: JsonValue) -> Self { AppError::BadRequest(value) }
	pub fn unauthorized(value: JsonValue) -> Self { AppError::Unauthorized(value) }
	pub fn unprocessable_entity(value: JsonValue) -> Self { AppError::UnprocessableEntity(value) }
}

impl From<JwtError> for AppError {
	fn from(jwt_error: JwtError) -> Self {
		match jwt_error.kind() {
			ErrorKind::InvalidToken => AppError::unauthorized(json!({
				"error": "Token is invalid"
			})),
			ErrorKind::InvalidIssuer => AppError::unauthorized(json!({
				"error": "Issuer is invalid",
			})),
			ErrorKind::ExpiredSignature => AppError::unauthorized(json!({
				"error": "Token Signature has expired"
			})),
			_ => AppError::internal_server_error(),
		}
	}
}

impl From<ValidationErrors> for AppError {
	fn from(error: ValidationErrors) -> Self {
		AppError::bad_request(json!({ "error": error.to_string() }))
	}
}

impl ResponseError for AppError {
	fn status_code(&self) -> StatusCode {
		match self {
			AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
			AppError::NotFound(_) => StatusCode::NOT_FOUND,
			AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
			AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
			AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
		}
	}

	fn error_response(&self) -> HttpResponse {
		match self {
			AppError::InternalServerError => HttpResponse::InternalServerError().finish(),
			AppError::NotFound(js) => HttpResponse::NotFound().json(js),
			AppError::Unauthorized(js) => HttpResponse::Unauthorized().json(js),
			AppError::BadRequest(js) => HttpResponse::BadRequest().json(js),
			AppError::UnprocessableEntity(js) => HttpResponse::UnprocessableEntity().json(js),
		}
	}
}

impl From<PoolError> for AppError {
	fn from(_e: PoolError) -> Self {
		AppError::internal_server_error()
	}
}

impl From<DieselError> for AppError {
	fn from(error: DieselError) -> Self {
		match error {
			DieselError::DatabaseError(kind, info) => {
				if let DatabaseErrorKind::UniqueViolation = kind {
					let message = info.details().unwrap_or_else(|| info.message()).to_string();
					return AppError::UnprocessableEntity(json!({ "error": message}))
				}
				AppError::InternalServerError
			}
			DieselError::NotFound => {
				AppError::NotFound(json!({ "error": "requested record was not found" }))
			}
			_ => AppError::InternalServerError
		}
	}
}