use std::{
    fmt::Debug,
};
use lambda_runtime::{LambdaEvent};
use serde_json::{json, Value};
use simple_error::SimpleError;

mod address;
mod mail;
mod sesevent;
mod aws;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(lambda_runtime::service_fn(handle_f)).await?;
    Ok(())
}

async fn handle_f(event: LambdaEvent<Value>) -> Result<Value, lambda_runtime::Error> {
    match handle(event).await {
      Ok(value) => {
          println!("All went well. Email forwarded.");
          Ok(value)
      },
      Err(e)   => {
          println!("An error occured: {:?}", e);
          Err(e)
      },
   }
}

async fn handle(event: LambdaEvent<Value>) -> Result<Value, lambda_runtime::Error> {
    let (payload, _context) = event.into_parts();
    println!("Received email {:?}", payload);
    let ses_event: sesevent::SesEvent = map_err(sesevent::SesEvent::from_json(&payload))?;
    println!("ses event details: {:?}", ses_event);
    let mut mail: mail::Mail = map_err(mail::Mail::from(ses_event.message_id.clone()))?;
    println!("mail object created");
    let store = map_err(oneml::aws::Store::default().await)?;
    println!("store object created");
    for address in map_err(address::Address::from_ses_event_destinations(&ses_event.destinations, &store).await)? {
        println!("Email {} forward {} => {}", ses_event.subject, address.from, address.to);
        map_err(mail.send(&address.from, &address.to, &address.reply_to, address.text, address.html).await)?;
        println!("Email forward complete");
    }
    Ok(json!({"status": "Success"}))
}

fn map_err<X, Y: Debug>(r: Result<X, Y>) -> Result<X, SimpleError> {
    r.map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))
}

