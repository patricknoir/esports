use serde::{Deserialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Credentials {
	#[validate(length(min = 1, message = "Email cannot be empty"))]
	pub email: String,
	#[validate(length(min = 1, message = "Password cannot be empty"))]
	pub password: String,
}