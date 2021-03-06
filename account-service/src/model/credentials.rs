use serde::{Deserialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Credentials {
	#[validate(email(message = "fails validation - is not a valid email address"))]
	pub email: String,
	#[validate(length(min = 1, message = "Password cannot be empty"))]
	pub password: String,
}