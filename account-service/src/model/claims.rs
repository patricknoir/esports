use uuid::Uuid;
use crate::model::user::User;
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use core_macro::{CanGenerateJwt, CanDecodeJwt};

#[derive(Debug, Serialize, Deserialize, CanGenerateJwt, CanDecodeJwt)]
pub struct Claims {
	pub user_id: Uuid,
	pub exp: i64,
	pub role: String,
}

impl Claims {
	pub fn new(user_id: Uuid, exp: i64, role: String) -> Self {
		Claims {
			user_id,
			exp,
			role
		}
	}
}

impl From<User> for Claims {
	fn from(user: User) -> Self {
		let exp = (Utc::now() + Duration::days(21)).timestamp();
		Claims {
			user_id: user.id,
			exp,
			role: user.role,
		}
	}
}
