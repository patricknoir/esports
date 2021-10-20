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

use core::data::DataService;
use core::actix::{Addr, SyncArbiter};
use std::env;
use core::prelude::*;


pub const ENVFILE_PATH: &str = "./account-service/.env";

pub const JWT_SECRET: &str = "JWT_SECRET";

pub struct AppState {
	pub data_service: Addr<DataService>,
}

impl AppState {
	pub async fn default() -> Self {
		AppState::with_db(env::var(DATABASE_URL).unwrap().into()).await
	}
	pub async fn with_db(db_url: String) -> Self {
		let addr = SyncArbiter::start(1, move || DataService::new(db_url.clone()));
		AppState::new(addr)

	}

	pub fn new(data_service: Addr<DataService>) -> Self {
		AppState {
			data_service
		}
	}
}

impl Clone for AppState {
	fn clone(&self) -> Self {
		AppState::new(self.data_service.clone())
	}
}