#[macro_use]
extern crate diesel;
extern crate serde;
extern crate core;

extern crate core_macro;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

pub mod api;
mod model;
mod schema;
mod data;
pub mod util;
mod io;

use core::data::DataService;
use core::actix::{Addr, SyncArbiter, Actor};
use crate::io::refresh_password_publisher::RefreshPasswordPublisher;


pub const ENVFILE_PATH: &str = "./account-service/.env";

pub const HASH_SECRET: &str = "HASH_SECRET";
pub const JWT_SECRET: &str = "JWT_SECRET";

pub struct AppState {
	pub data_service: Addr<DataService>,
	pub refresh_password_publisher: Addr<RefreshPasswordPublisher>
}

impl AppState {
	pub async fn default() -> Self {
		AppState::with_config(util::Config::default()).await
	}
	pub async fn with_config(config: util::Config) -> Self {
		let kafka_servers = config.kafka_servers.as_str();
		let db_url = config.database_url.clone();
		let ds_addr = SyncArbiter::start(1, move || DataService::new(db_url.clone()));
		let rpp_addr = RefreshPasswordPublisher::new(
			config.refresh_password_topic.clone(),
			core::messages::kafka::create_producer(kafka_servers)
		).start();
		AppState::new(ds_addr, rpp_addr)
	}

	pub fn new(data_service: Addr<DataService>, refresh_password_publisher: Addr<RefreshPasswordPublisher>) -> Self {
		AppState {
			data_service,
			refresh_password_publisher
		}
	}
}

impl Clone for AppState {
	fn clone(&self) -> Self {
		AppState::new(self.data_service.clone(), self.refresh_password_publisher.clone())
	}
}