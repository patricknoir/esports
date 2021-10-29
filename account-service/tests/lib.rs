#[macro_use]
extern crate diesel_migrations;

extern crate actix_rt;

use core::actix_web;
use core::actix_web::App;
use core::actix_web::web::Data;
use account_service::AppState;
use core::actix_web::test::*;
use testcontainers::clients::Cli;

mod util;

#[actix_rt::test]
async fn test_health_get() {
	let docker = Cli::default();
	let container = util::setup(&docker).unwrap();

	let app_state: AppState = AppState::default().await;
	let mut app = init_service(
		App::new()
			.app_data(Data::new(app_state.clone()))
			.configure(account_service::api::routes)
	).await;
	let req = TestRequest::get().uri("/health").to_request();
	let resp = call_service(&mut app, req).await;

	util::teardown(&docker, container.id());

	assert!(resp.status().is_success());
}