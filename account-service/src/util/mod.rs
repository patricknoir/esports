use chrono::Duration;
use crate::{JWT_SECRET, HASH_SECRET};
use core::prelude::{DATABASE_URL, BIND_ADDRESS};

const SESSION_TOKEN_DURATION_DAYS: &str = "SESSION_TOKEN_DURATION_DAYS";
const REFRESH_PASSWORD_DURATION_MINS: &str = "REFRESH_PASSWORD_DURATION_MINS";

const REFRESH_PASSWORD_TOPIC: &str = "REFRESH_PASSWORD_TOPIC";
const KAFKA_SERVERS: &str = "KAFKA_SERVERS";

#[derive(Debug)]
pub struct Config {
	pub jwt_secret: String,
	pub hash_secret: String,
	pub database_url: String,
	pub bind_address: String,
	pub session_token_duration: Duration,
	pub refresh_password_duration: Duration,
	pub refresh_password_topic: String,
	pub kafka_servers: String
}

impl Default for Config {
	fn default() -> Self {
		let jwt_secret = std::env::var(JWT_SECRET).expect("JWT_SECRET environment variable not found");
		let hash_secret = std::env::var(HASH_SECRET).expect("HASH_SECRET environment variable not found");
		let database_url = std::env::var(DATABASE_URL).expect("DATABASE_URL environment variable not found");
		let bind_address = std::env::var(BIND_ADDRESS).expect("BIND_ADDRESS environment variable not found");
		let days: i64 =  std::env::var(SESSION_TOKEN_DURATION_DAYS).expect("SESSION_TOKEN_DURATION_DAYS environment variable not found").parse().expect("SESSION_TOKEN_DURATION_DAYS cannot be parsed");
		let mins: i64 =  std::env::var(REFRESH_PASSWORD_DURATION_MINS).expect("REFRESH_PASSWORD_DURATION_MINS environment variable not found").parse().expect("REFRESH_PASSWORD_DURATION_MINS cannot be parsed");
		let session_token_duration = Duration::days(days);
		let refresh_password_duration = Duration::minutes(mins);
		let refresh_password_topic = std::env::var(REFRESH_PASSWORD_TOPIC).expect("REFRESH_PASSWORD_TOPIC environment variable not found");
		let kafka_servers = std::env::var(KAFKA_SERVERS).expect("KAFKA_SERVERS environment variable not found");

		Config {
			jwt_secret,
			hash_secret,
			database_url,
			bind_address,
			session_token_duration,
			refresh_password_duration,
			refresh_password_topic,
			kafka_servers
		}
	}
}
