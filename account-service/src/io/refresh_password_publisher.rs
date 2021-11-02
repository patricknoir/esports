use core::rdkafka::producer::{FutureProducer, FutureRecord};
use core::actix::{Actor, Context, Handler};
use uuid::Uuid;
use core::actix::Message;
use core::prelude::*;
use core::error::AppError;

pub struct RefreshPasswordPublisher {
	pub topic: String,
	pub publisher: FutureProducer
}

impl Actor for RefreshPasswordPublisher {
	type Context = Context<Self>;
}

impl RefreshPasswordPublisher {
	pub fn new(topic: String, publisher: FutureProducer) -> Self {
		RefreshPasswordPublisher {
			topic,
			publisher
		}
	}
}

#[derive(Debug)]
pub struct RefreshPasswordMessage {
	pub user_id: Uuid,
	pub email: String,
	pub token: String
}

impl Message for RefreshPasswordMessage {
	type Result = Result<()>;
}

impl Handler<RefreshPasswordMessage> for RefreshPasswordPublisher {
	type Result = Result<()>;

	fn handle(&mut self, msg: RefreshPasswordMessage, _ctx: &mut Self::Context) -> Self::Result {
		let result = self.publisher.send_result(
			FutureRecord::to(self.topic.as_str())
				.payload(msg.token.as_str())
				.key(msg.email.as_str())
		);

		if let Err((err, _)) = result {
			dbg!(err);
			return Err(AppError::internal_server_error());
		}

		Ok(())
	}
}