use argonautica::{Hasher, Verifier};
use crate::prelude::*;
use crate::error::AppError;

pub fn hash_password(email: &str, plain_password: &str, secret: &str) -> Result<String> {
	let mut hasher = Hasher::default();
	hasher
		.with_salt(email)
		.with_password(plain_password)
		.with_secret_key(secret)
		.hash()
		.map_err(|_| AppError::internal_server_error())
}

pub fn validate_password(hash: &str, password: &str, secret: &str) -> Result<bool> {
	dbg!(hash, password);
	let mut verifier = Verifier::default();
	let result = verifier
		.with_hash(hash)
		.with_password(password)
		.with_secret_key(secret)
		.verify()
		.map_err(|_| AppError::internal_server_error());

	dbg!(&result);

	result
}
