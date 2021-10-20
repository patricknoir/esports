#[macro_use]
extern crate core_macro;

mod account;

use std::error::Error;
use tokio;

async fn startup() -> Result<(), Box<dyn Error>> {
	println!("Startup!");
	Ok(())
}

async fn teardown() -> Result<(), Box<dyn Error>> {
	println!("Teardown!");
	Ok(())
}

pub struct IntegrationTest {
	pub name: &'static str,
	pub test_fn: Box<dyn Fn()->()>,
}

inventory::collect!(IntegrationTest);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	startup().await.expect("failed during startup");

	for t in inventory::iter::<IntegrationTest> {
		println!("Running test: {}", t.name);
		(t.test_fn)();
	}

	teardown().await
}