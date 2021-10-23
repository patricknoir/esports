extern crate core_macro;
#[macro_use]
extern crate diesel_migrations;

mod account;
mod helper;

use std::error::Error;
use testcontainers::{clients, Docker, Container};
use tokio;

async fn startup<'a>(docker: &'a clients::Cli) -> Result<Container<'a, clients::Cli, testcontainers::images::postgres::Postgres>, Box<dyn Error>> {
	println!("Startup!");

	let node = helper::run_image_esports_db(&docker)?;

	Ok(node)
}

async fn teardown(docker: &clients::Cli, container_id: &str) -> Result<(), Box<dyn Error>> {
	println!("Teardown!");
	docker.stop(container_id);
	Ok(())
}

pub struct IntegrationTest {
	pub name: &'static str,
	pub test_fn: Box<dyn Fn()->()>,
}

inventory::collect!(IntegrationTest);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let docker: clients::Cli = clients::Cli::default();

	let container: Container<clients::Cli, testcontainers::images::postgres::Postgres> = startup(&docker).await.expect("failed during startup");


	for t in inventory::iter::<IntegrationTest> {
		println!("Running test: {}", t.name);
		(t.test_fn)();
	}

	teardown(&docker, container.id()).await
}