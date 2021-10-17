use serde::Serialize;
use actix_web::Responder;
use actix_web::HttpRequest;
use actix_http::Error;
use actix_web::HttpResponse;
use core::future::{Ready, ready};
use crate::error::AppError;

/// Represents a response returned by the application public API
/// A response has a code which maps directly into the HTTP RESPONSE CODES,
/// a token only if this is creating a new session (Registration, Login),
/// a body which can be serialised into json format.
/// AppResponse provides a series of utility factory functions for all the
/// responses you need to return.
#[derive(Debug, Serialize)]
pub struct AppResponse<T: Serialize> {
	code: u64,
	pub token: Option<String>,
	pub body: Option<T>,
}

pub const HTTP_SUCCESS: u64 = 200;
pub const HTTP_CREATED: u64 = 201;

pub const HTTP_HEADER_AUTHORIZATION: &str = "Authorization";

impl<T: Serialize> AppResponse<T> {
	fn new_instance(code: u64, token: Option<String>, body: Option<T>) -> Self {
		AppResponse {
			code,
			token,
			body
		}
	}

	/// Return an AppResponse for an HTTP_CREATED response with an optionally body and optionally a token representing the session.
	pub fn created(token: Option<String>, entity: Option<T>) -> Self {
		AppResponse::new_instance(HTTP_CREATED, token, entity)
	}

	/// Return an AppResponse for an HTTP_SUCCESS response with a body and a token representing the session.
	/// If you don't want to return the token use the body() constructor.
	pub fn response(token: String, body: T) -> Self {
		AppResponse::new_instance(HTTP_SUCCESS, Some(token), Some(body))
	}

	/// Return an AppResponse for an HTTP_SUCCESS with empty payload.
	pub fn empty() -> Self {
		AppResponse::new_instance(HTTP_SUCCESS, None, None)
	}

	/// Return an AppResponse for an HTTP_SUCCESS with a token as body (Mainly used for Login)
	pub fn token(token: String) -> Self {
		AppResponse::new_instance(HTTP_SUCCESS, Some(token), None)
	}

	/// Return an AppResponse for an HTTP_SUCCESS response with a body.
	pub fn body(t: T) -> Self {
		AppResponse::new_instance(HTTP_SUCCESS, None, Some(t))
	}

	/// Return an AppResponse for an HTTP_CREATED empty response.
	pub fn empty_created() -> Self {
		AppResponse::new_instance(HTTP_CREATED, None, None)
	}
}

impl<T: Serialize> Responder for AppResponse<T> {
	type Error = Error;
	type Future = Ready<Result<HttpResponse, Error>>;

	fn respond_to(self, _req: &HttpRequest) -> Self::Future {
		let response = match(self.code, self.token, self.body) {
			(200, Some(token), Some(payload)) => Ok(HttpResponse::Ok().header(HTTP_HEADER_AUTHORIZATION, format!("{} {}", "Bearer", token)).json(payload)),
			(200, None, Some(payload)) => Ok(HttpResponse::Ok().json(payload)),
			(201, Some(token), Some(payload)) => Ok(HttpResponse::Created().header(HTTP_HEADER_AUTHORIZATION, format!("{} {}", "Bearer", token)).json(payload)),
			(201, None, Some(payload)) => Ok(HttpResponse::Created().json(payload)),
			(200, Some(token), None) => Ok(HttpResponse::Ok().header(HTTP_HEADER_AUTHORIZATION, format!("{} {}", "Bearer", token)).finish()),
			(200, None, None) => Ok(HttpResponse::Ok().finish()),
			(201, Some(token), None) => Ok(HttpResponse::Created().header(HTTP_HEADER_AUTHORIZATION, format!("{} {}", "Bearer", token)).finish()),
			(201, None, None) => Ok(HttpResponse::Created().finish()),
			_ =>  Err(AppError::internal_server_error().into()),
		};
		ready(response)
	}
}
