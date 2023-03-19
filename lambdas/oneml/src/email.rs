use std::{
  error::Error,
  collections::HashMap,
};
use serde::{Serialize, Deserialize};
use simple_error::SimpleError;
use async_trait::async_trait;

use crate::Identity;

#[derive(Serialize, Deserialize)]
pub enum Status {
  Forward,
  ForwardAsText,
  Block,
}
impl Status {
  pub fn from_str(v: &str) -> Result<Status, Box<dyn Error>> {
    match v {
      "Forward"       => Ok(Status::Forward),
      "ForwardAsText" => Ok(Status::ForwardAsText),
      "Block"         => Ok(Status::Block),
      _               => Err(Box::new(SimpleError::new(format!("{} is not a valid status", v).as_str()))),
    }
  }

  pub fn to_str(status: &Status) -> Result<String, Box<dyn Error>> {
    match status {
      Status::Forward       => Ok("Forward".to_string()),
      Status::ForwardAsText => Ok("ForwardAsText".to_string()),
      Status::Block         => Ok("Block".to_string()),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Email {
  pub email: String,
  pub user_id: String,
  pub status: Status,
}

impl Email {
  pub fn new(email: String, user_id: String) -> Result<Email, Box<dyn Error>> {
    Ok(Email { email, user_id, status: Status::Forward } )
  }
  pub fn status(mut self, status: Status) -> Result<Email, Box<dyn Error>> {
    self.status = status;
    Ok(self)
  }
}

#[async_trait]
pub trait Store {
  async fn email_list_from_user_id(&self, user_id: &str) -> Result<Vec<Email>, Box<dyn Error>>;
  async fn update_email(&self, email: Email) -> Result<Email, Box<dyn Error>>;
  async fn from_address(&self, email_address: &str) -> Result<Option<Email>, Box<dyn Error>>;
  async fn save_email(&self, email: Email) -> Result<Email, Box<dyn Error>>;
}

impl Email {
  pub async fn save<T>(self, store: &T) -> Result<Email, Box<dyn Error>>
  where T: Store {
    store.save_email(self).await
  }

  pub async fn from_address<T>(email_address: &str, store: &T) -> Result<Option<Email>, Box<dyn Error>>
  where T: Store {
    store.from_address(email_address).await
  }

  pub async fn list_from_identity<T>(identity: &Identity, store: &T) -> Result<Vec<Email>, Box<dyn Error>>
  where T: Store {
    store.email_list_from_user_id(identity.id.as_str()).await
  }

  pub async fn update_from_identity<T>(identity: &Identity, store: &T, email: String, update: HashMap<String, String>) -> Result<Email, Box<dyn Error>>
  where T: Store {
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

