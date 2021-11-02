use core_macro::async_integration_test;
use core::actix_web::client::Client;

#[async_integration_test]
pub async fn base_account_test() {
	println!("Running async tests");
	let client = Client::default();

	// Create request builder and send request
	let response = client.get("http://localhost:9070/health")
		.header("User-Agent", "actix-web/3.0")
		.send()     // <- Send request
		.await;     // <- Wait for response

	println!("Response: {:?}", response);
}

