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
    env_logger::init();
    lambda_runtime::run(lambda_runtime::service_fn(handle_f)).await?;
    Ok(())
}

async fn handle_f(event: LambdaEvent<Value>) -> Result<Value, lambda_runtime::Error> {
    match handle(event).await {
      Ok(value) => {
          log::info!("All went well. Email forwarded.");
          Ok(value)
      },
      Err(e)   => {
          log::error!("An error occured: {:?}", e);
          oneml::sns_notify(format!("{e:?}"));
          Err(e)
      },
   }
}

async fn handle(event: LambdaEvent<Value>) -> Result<Value, lambda_runtime::Error> {
    let (payload, _context) = event.into_parts();
    log::debug!("Received email {:?}", payload);
    let ses_event: sesevent::SesEvent = map_err(sesevent::SesEvent::from_json(&payload))?;
    log::debug!("ses event details: {:?}", ses_event);
    let mut mail: mail::Mail = map_err(mail::Mail::from(ses_event.message_id.clone()))?;
    log::debug!("mail object created");
    let store = map_err(oneml::aws::Store::default().await)?;

    use oneml::derive_sql::traits::{Table, Insert};
    let mut conn = map_err(oneml::db::connection())?;
    let db_conn  = &mut conn;
    let email_db = oneml::db::tables::SqlEmail::default();
    map_err(email_db.create_if_not_exist(db_conn))?;

    log::debug!("store object created");
    for address in map_err(address::Address::from_ses_event_destinations(&ses_event.destinations, &store).await)? {
      if address.forward {
        log::debug!("Email {} forward {} => {}", ses_event.subject, address.from, address.to);
        map_err(mail.send(&address.from, &address.to, &address.reply_to, address.text, address.html).await)?;
        log::debug!("Email forward complete");
      } else {
        log::debug!("Email {subject} from {from}: skip", subject = ses_event.subject, from = address.from);
      }

      log::debug!("Record new entry in database");
      let email = oneml::db::tables::Email::from_id_message_id_subject_from_to_forwarded(address.account_id.as_str(),
        ses_event.message_id.as_str(), ses_event.subject.as_str(), ses_event.source.as_str(), address.from.as_str(), address.forward);
      map_err(email_db.insert(db_conn, &email))?;
      log::debug!("Database entry recorded");
    }

    Ok(json!({"status": "Success"}))
}

fn map_err<X, Y: Debug>(r: Result<X, Y>) -> Result<X, SimpleError> {
    r.map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))
}

