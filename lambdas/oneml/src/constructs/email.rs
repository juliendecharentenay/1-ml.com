use super::*;

use std::{
  collections::HashMap,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
  Forward,
  ForwardAsText,
  Block,
}
impl Status {
  pub fn from_str(v: &str) -> Result<Status> {
    match v {
      "Forward"       => Ok(Status::Forward),
      "ForwardAsText" => Ok(Status::ForwardAsText),
      "Block"         => Ok(Status::Block),
      _               => Err(format!("{} is not a valid status", v).as_str().into()),
    }
  }

  pub fn to_str(status: &Status) -> Result<String> {
    match status {
      Status::Forward       => Ok("Forward".to_string()),
      Status::ForwardAsText => Ok("ForwardAsText".to_string()),
      Status::Block         => Ok("Block".to_string()),
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Email {
  pub email: String,
  pub user_id: String,
  pub status: Status,
}

impl Email {
  pub fn new(email: String, user_id: String) -> Result<Email> {
    Ok(Email { email, user_id, status: Status::Forward } )
  }
  pub fn status(mut self, status: Status) -> Result<Email> {
    self.status = status;
    Ok(self)
  }
  pub fn from_email_userid_status(email: &str, user_id: &str, status: Status) -> Email {
    Email { email: email.to_string(), user_id: user_id.to_string(), status }
  }
}

impl Email {
  pub async fn save<T>(self, store: &T) -> Result<Email>
  where T: traits::store::EmailStore {
    store.save_email(self).await
  }

  pub async fn from_address<T>(email_address: &str, store: &T) -> Result<Option<Email>>
  where T: traits::store::EmailStore {
    store.from_address(email_address).await
  }

  pub async fn list_from_identity<T>(identity: &Identity, store: &T) -> Result<Vec<Email>>
  where T: traits::store::EmailStore {
    store.email_list_from_user_id(identity.id.as_str()).await
  }

  pub async fn update_from_identity<T>(identity: &Identity, store: &T, email: String, update: HashMap<String, String>) -> Result<Email>
  where T: traits::store::EmailStore {
    let mut email = Email::new(email, identity.id.clone())?;
    let mut updated = false;
    if let Some(status) = update.get("status") {
      email.status = Status::from_str(status.as_str())?;
      updated = true;
    }

    if updated {
      store.update_email(email).await
    } else {
      Ok(email)
    }
  }
}

