use core::actix_web::{Responder, HttpResponse};
use core::actix_web;

#[actix_web::get("/health")]
pub async fn health_check() -> impl Responder {
	HttpResponse::Ok().finish()
}