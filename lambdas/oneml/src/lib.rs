 
pub mod response;
pub mod aws;
pub mod account;
pub mod email;
pub mod error;
pub mod db;

mod identity;
pub use identity::{Identity, IdentityBuilder};

pub use derive_sql;


/// Retrieve a connection to the database
pub fn get_database_connection() -> error::Result<rusqlite::Connection> {
  Ok(rusqlite::Connection::open_in_memory()?)
}

/// Notify to an SNS topic
pub fn sns_notify(topic_arn: String, msg: String) -> error::Result<()> {
  let config = futures::executor::block_on(aws_config::load_defaults(aws_config::BehaviorVersion::latest()));
  let client = aws_sdk_sns::Client::new(&config);
  futures::executor::block_on(
    client
    .publish()
    .topic_arn(topic_arn)
    .message(msg)
    .send()
  ).map_err(error::Error::from)?;
  Ok(())
}
