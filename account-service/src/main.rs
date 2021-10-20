use account_service::{AppState, ENVFILE_PATH, api};
use core::actix_web::{HttpServer, App};
use core::actix_web::web::Data;
use std::env;
use core::prelude::BIND_ADDRESS;
use core::actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let state: AppState = AppState::default().await;
    start_server(state).await
}

pub async fn start_server(app_state: AppState) -> std::io::Result<()> {
    // init_logger();
    HttpServer::new(move || {
        App::new()
          .app_data(Data::new(app_state.clone()))
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