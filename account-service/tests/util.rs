use std::collections::HashMap;
use testcontainers::clients::Cli;
use testcontainers::{Image, Container, Docker, RunArgs};
use std::error::Error;
use diesel::r2d2::{ConnectionManager, ManageConnection};
use diesel::PgConnection;
use core::prelude::DATABASE_URL;
use diesel_migrations::*;

embed_migrations!();

const DB_NAME: &str = "esports";
const DB_USER: &str = "postgres";
const DB_PASS: &str = "postgres";
const DB_PORT_ORIGIN: u16 = 5432;
const DB_PORT_DEST: u16 = 5432;

const ENV_POSTGRES_DB: &str = "POSTGRES_DB";
const ENV_POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
const ENV_POSTGRES_USER: &str = "POSTGRES_USER";


pub fn setup<'a>(docker: &'a Cli) -> Result<Container<'a, Cli, testcontainers::images::postgres::Postgres>, Box<dyn Error>> {
	println!("Starting");
	dotenv::dotenv().ok();
	let db_url = get_var(DATABASE_URL).expect("DATABASE_URL environment variable not found");
	let mut env_vars: HashMap<String, String> = HashMap::new();
	env_vars.insert(ENV_POSTGRES_DB.to_string(), DB_NAME.to_string());
	env_vars.insert(ENV_POSTGRES_USER.to_string(), DB_USER.to_string());
	env_vars.insert(ENV_POSTGRES_PASSWORD.to_string(), DB_PASS.to_string());

	let postgres_image = testcontainers::images::postgres::Postgres::default().with_env_vars(env_vars);

	let node = run_image(docker, Some(DB_NAME.to_string()), postgres_image, Some((DB_PORT_ORIGIN, DB_PORT_DEST)))?;
	let manager = ConnectionManager::<PgConnection>::new(db_url);
	let conn = manager.connect()?;

	embedded_migrations::run(&conn)?;
	Ok(node)
}

pub fn teardown(docker: &Cli, container_id: &str) -> Result<(), Box<dyn Error>> {
	println!("Terimanted");
	docker.stop(container_id);
	Ok(())
}

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

pub fn get_var(name: &str) -> Option<String> {
	std::env::var(name).ok()
}