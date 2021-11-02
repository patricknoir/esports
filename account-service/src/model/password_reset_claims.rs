use serde::{Serialize, Deserialize};
use core_macro::{CanGenerateJwt, CanDecodeJwt};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, CanGenerateJwt, CanDecodeJwt)]
pub struct PasswordResetClaims {
	pub user_id: Uuid,
	pub email: String,
	pub exp: i64
}

impl PasswordResetClaims {
	pub fn new(user_id: Uuid, email: String, exp: i64) -> Self {
		PasswordResetClaims {
			user_id,
			email,
			exp
		}
	}
}
