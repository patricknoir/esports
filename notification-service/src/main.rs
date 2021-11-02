mod util;
mod model;
mod resources;

extern crate core_macro;
extern crate serde;

use core::rdkafka::{ClientConfig, Message as KafkaMessage};
use core::rdkafka::consumer::stream_consumer::StreamConsumer;
use core::rdkafka::consumer::Consumer;
use futures::stream::TryStreamExt;
use core::prelude::*;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::time::Duration;

const ENVFILE_PATH: &str = "./notification-service/.env";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path(ENVFILE_PATH).ok();
    run_process().await;
    Ok(())
}

async fn run_process() {
    let config = util::Config::default();

    let consumer: StreamConsumer = ClientConfig::new()
      .set("group.id", config.kafka_group_id.as_str())
      .set("bootstrap.servers", config.kafka_servers.as_str())
      .set("enable.partition.eof", "false")
      // .set("session.timeout.ms", "6000")
      .set("enable.auto.commit", "true")
      .create()
      .expect("Consumer creation failed");

    consumer
      .subscribe(&[config.refresh_password_topic.as_str()])
      .expect("Can't subscribe to specified topic");

    let template = include_str!("resources/template.html");

    let stream_processor = consumer.stream().try_for_each(|msg|  {
        // let email = str::from_bytes(msg.key().unwrap()).expect("Failed decoding the message key [email]");
        // let token = str::from_bytes(msg.payload().unwrap()).expect("Failed decoding the message payload [token]");
        let email = String::from_utf8(Vec::from(msg.key().unwrap())).expect("Failed decoding the message key [email]");
        let token = String::from_utf8(Vec::from(msg.payload().unwrap())).expect("Failed decoding the message payload [token]");
        async move {
            // tokio::spawn(async move {
            //     let _ =
            //       tokio::task::spawn_blocking(move || send_mail(email.clone(), token.clone()))
            //         .await
            //         .expect("failed to wait for send_mail computation");
            // });
            let _ = send_mail(email.clone(), token.clone(), template.to_string()).await;
            Ok(())
        }
    });

    stream_processor.await.expect("stream processing failed");
}

async fn send_mail(email_addr: String, token: String, template: String) -> Result<()> {
    println!("SENDING EMAIL");
    let body = template.replace("{{{--REPLACE-ME--}}}", format!("http://localhost:9080/password/reset?token={}", token).as_str());
    let email = Message::builder()
      .from("esports@gmail.com".parse().unwrap())
      .reply_to("esports@gmail.com".parse().unwrap())
      .to(email_addr.parse().unwrap())
      .subject("Password Reset")
      .header(lettre::message::header::ContentType::TEXT_HTML)
      .body(body)
      .unwrap();

    let creds = Credentials::new("esports".to_string(), "password".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
      .unwrap()
      .credentials(creds)
      .timeout(Some(Duration::new(10, 0)))
      .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }

    Ok(())
}