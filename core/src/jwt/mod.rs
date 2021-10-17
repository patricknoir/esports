use crate::prelude::*;
use jsonwebtoken::{TokenData, Header, EncodingKey, encode};
use serde::Serialize;

pub trait CanGenerateJwt: Serialize + Sized {
	fn to_jwt(&self, secret: String) -> Result<String> {
		let header: Header = Header::default();
		let key = EncodingKey::from_secret(secret.as_bytes());
		encode(&header, self, &key).map_err(|e| e.into())
	}
}

pub trait CanDecodeJwt<T> {
	fn decode_jwt(&self, secret: String) -> Result<TokenData<T>>;
}