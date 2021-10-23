use testcontainers::clients::Cli;
use std::collections::HashMap;
use testcontainers::{Container, Docker, Image, RunArgs};
use std::error::Error;
use diesel::r2d2::{ConnectionManager, ManageConnection};
use diesel::PgConnection;
use diesel_migrations::*;


embed_migrations!("../account-service/migrations");


const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/esports";
const PORT_ORIGIN: u16 = 5432;
const PORT_DEST: u16 = 5432;
const DB_NAME: &str = "esports";
const DB_USER: &str = "postgres";
const DB_PASS: &str = "postgres";

const POSTGRES_DB: &str = "POSTGRES_DB";
const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
const POSTGRES_USER: &str = "POSTGRES_USER";


pub fn run_image<'a, I: Image>(docker: &'a Cli, name: Option<String>, image:I, port_mapping: Option<(u16, u16)>) -> Result<Container<'a, Cli, I>, Box<dyn Error>> {
	let mut run_args = RunArgs::default();

	run_args = if let Some(image_name) = name {
		run_args.with_name(image_name)
	} else {
		run_args
	};

	run_args = if let Some(mapping) = port_mapping {
		run_args.with_mapped_port(mapping)
	} else {
		run_args
	};

	Ok(docker.run_with_args(image, run_args))
}

pub fn run_image_esports_db<'a>(docker: &'a Cli) -> Result<Container<'a, Cli, testcontainers::images::postgres::Postgres>, Box<dyn Error>> {
	let mut env_vars: HashMap<String, String> = HashMap::new();
	env_vars.insert(POSTGRES_DB.to_string(), DB_NAME.to_string());
	env_vars.insert(POSTGRES_USER.to_string(), DB_USER.to_string());
	env_vars.insert(POSTGRES_PASSWORD.to_string(), DB_PASS.to_string());

	let postgres_image = testcontainers::images::postgres::Postgres::default().with_env_vars(env_vars);

	let node = run_image(docker, Some("esports".to_string()), postgres_image, Some((PORT_ORIGIN, PORT_DEST)))?;
	let manager = ConnectionManager::<PgConnection>::new(DB_URL);
	let conn = manager.connect()?;

	embedded_migrations::run(&conn)?;
	Ok(node)
}