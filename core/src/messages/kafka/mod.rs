use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;


pub fn create_producer(kafka_servers: &str) -> FutureProducer {
	ClientConfig::new()
		.set("bootstrap.servers", kafka_servers)
		.create()
		.expect("Failed creating Kafka FutureProducer")
}