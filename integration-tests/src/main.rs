extern crate core_macro;
#[macro_use]
extern crate diesel_migrations;

mod account;

use std::error::Error;
use testcontainers::{clients, images::postgres, Docker, RunArgs, Container};
use tokio;
use diesel_migrations::*;
use diesel::r2d2::{ConnectionManager, ManageConnection};
use diesel::PgConnection;
use std::collections::HashMap;

embed_migrations!("../account-service/migrations");


async fn startup<'a>(docker: &'a clients::Cli) -> Result<Container<'a, clients::Cli, postgres::Postgres>, Box<dyn Error>> {
	println!("Startup!");
	let args = RunArgs::default().with_mapped_port((5432, 5432));
	let mut envs = HashMap::new();
	envs.insert("POSTGRES_DB".to_string(), "esports".to_string());
	envs.insert("POSTGRES_PASSWORD".to_string(), "postgres".to_string());
	envs.insert("POSTGRES_USER".to_string(), "postgres".to_string());
	let node = docker.run_with_args(postgres::Postgres::default().with_env_vars(envs), args);

	let db_url = "postgres://postgres:postgres@localhost:5432/esports";
	let manager = ConnectionManager::<PgConnection>::new(db_url);
	let conn = manager.connect()?;

	embedded_migrations::run(&conn)?;
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

	let container: Container<clients::Cli, postgres::Postgres> = startup(&docker).await.expect("failed during startup");


	for t in inventory::iter::<IntegrationTest> {
		println!("Running test: {}", t.name);
		(t.test_fn)();
	}

	teardown(&docker, container.id()).await
}