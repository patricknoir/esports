mod api;
mod model;
mod schema;
mod data;

#[macro_use]
extern crate diesel;
extern crate serde;
extern crate core;

extern crate core_macro;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;


use core::data::DataService;
use core::actix::{Addr, SyncArbiter};
use std::env;
use core::actix_web::{HttpServer, App};
use core::actix_web::web::Data;
use core::prelude::*;
use core::actix_web;

const ENVFILE_PATH: &str = "./account-service/.env";

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let state: AppState = AppState::default().await;

    HttpServer::new(move || {
        App::new()
          .app_data(Data::new(state.clone()))
          .configure(api::routes)
    })
      .bind(env::var(BIND_ADDRESS).unwrap())?
      .run()
      .await
}

fn init_logger() {
    dotenv::from_path(ENVFILE_PATH).ok();
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "esports=debug,actix_web=info");
    }
    env_logger::init();
}