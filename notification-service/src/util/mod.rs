const KAFKA_SERVERS: &str = "KAFKA_SERVERS";
const REFRESH_PASSWORD_TOPIC: &str = "REFRESH_PASSWORD_TOPIC";
const KAFKA_GROUP_ID: &str = "KAFKA_GROUP_ID";

const HASH_SECRET: &str = "HASH_SECRET";

pub struct Config {
	pub kafka_servers: String,
	pub refresh_password_topic: String,
	pub kafka_group_id: String,
	pub hash_secret: String
}

impl Default for Config {
	fn default() -> Self {
		Config {
			kafka_servers: std::env::var(KAFKA_SERVERS).expect("KAFKA_SERVERS environment variable is not defined."),
			refresh_password_topic: std::env::var(REFRESH_PASSWORD_TOPIC).expect("REFRESH_PASSWORD_TOPIC environment variable is not defined."),
			kafka_group_id: std::env::var(KAFKA_GROUP_ID).expect("KAFKA_GROUP_ID environment variable is not defined."),
			hash_secret: std::env::var(HASH_SECRET).expect("HASH_SECRET environment variable is not defined.")
		}
	}
}