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

const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/esports";
const PORT_ORIGIN: u16 = 5432;
const PORT_DEST: u16 = 5432;
const DB_NAME: &str = "esports";
const DB_USER: &str = "postgres";
const DB_PASS: &str = "postgres";

const POSTGRES_DB: &str = "POSTGRES_DB";
const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
const POSTGRES_USER: &str = "POSTGRES_USER";


embed_migrations!("../account-service/migrations");


async fn startup<'a>(docker: &'a clients::Cli) -> Result<Container<'a, clients::Cli, postgres::Postgres>, Box<dyn Error>> {
	println!("Startup!");
	let args = RunArgs::default().with_mapped_port((PORT_ORIGIN, PORT_DEST));
	let mut envs = HashMap::new();
	envs.insert(POSTGRES_DB.to_string(), DB_NAME.to_string());
	envs.insert(POSTGRES_USER.to_string(), DB_USER.to_string());
	envs.insert(POSTGRES_PASSWORD.to_string(), DB_PASS.to_string());
	let node = docker.run_with_args(postgres::Postgres::default().with_env_vars(envs), args);

	let manager = ConnectionManager::<PgConnection>::new(DB_URL);
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