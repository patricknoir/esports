extern crate core_macro;
#[macro_use]
extern crate diesel_migrations;

mod account;
mod helper;

use std::error::Error;
use testcontainers::{clients, Docker, Container};
use core::actix_web::HttpServer;
use core::actix_web::web::Data;
use account_service::{AppState, util};
use core::actix_web::App;
use core::actix_web;

const BIND_ADDRESS: &str = "0.0.0.0:9070";

async fn startup<'a>(docker: &'a clients::Cli) -> Result<Container<'a, clients::Cli, testcontainers::images::postgres::Postgres>, Box<dyn Error>> {
	println!("Startup!");

	let node = helper::run_image_esports_db(&docker)?;
	let app_state: AppState = AppState::with_config(util::Config::default()).await;

	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(app_state.clone()))
			.configure(account_service::api::routes)
	})
		.bind(BIND_ADDRESS)?
		.run();

	Ok(node)
}

async fn teardown(docker: &clients::Cli, container_id: &str) -> Result<(), Box<dyn Error>> {
	println!("Teardown!");
	docker.stop(container_id);
	Ok(())
}

pub struct IntegrationTest {
	pub name: &'static str,
	pub test_fn: Box<dyn Fn() -> ()>,
}

pub struct IntTest<C, F> where
	C: Fn() -> F,
	F: std::future::Future
{
	pub name: &'static str,
	pub test_fn: C,
}


inventory::collect!(IntegrationTest);

// #[tokio::main]
#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let docker: clients::Cli = clients::Cli::default();

	let container: Container<clients::Cli, testcontainers::images::postgres::Postgres> = startup(&docker).await.expect("failed during startup");

	for t in inventory::iter::<IntegrationTest> {
		println!("Running tests: {}", t.name);
		// core::actix_web::web::block(t.test_fn).await?;
		(t.test_fn)();
	}

	// let mut _s = String::from("");
	// std::io::stdin().read_line(&mut _s)?;

	teardown(&docker, container.id()).await
}
