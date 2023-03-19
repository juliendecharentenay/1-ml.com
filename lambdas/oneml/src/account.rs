use std::{
  error::Error,
  collections::HashMap,
};
use serde::{Serialize, Deserialize};
use simple_error::SimpleError;
use async_trait::async_trait;
use chrono::TimeZone;

use crate::Identity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
  Active,
  Deleted,
}

impl Status {
  pub fn from_str(v: &str) -> Result<Status, Box<dyn Error>> {
    match v {
      "Active"  => Ok(Status::Active),
      "Deleted" => Ok(Status::Deleted),
      _         => Err(Box::new(SimpleError::new(format!("{} is not a valid status", v).as_str()))),
    }
  }

  pub fn to_str(status: &Status) -> Result<String, Box<dyn Error>> {
    match status {
      Status::Active => Ok("Active".to_string()),
      Status::Deleted => Ok("Deleted".to_string()),
    }
  }
}

#[cfg(test)]
mod status {
  use super::*;

  #[test]
  fn str_conversion() -> Result<(), Box<dyn Error>> {
    assert!(std::matches!(Status::from_str("Active")?, Status::Active));
    assert!(std::matches!(Status::from_str("Deleted")?, Status::Deleted));
    assert!(Status::from_str("Banana").is_err());

    assert!(Status::to_str(&Status::Active)?.eq("Active"));
    assert!(Status::to_str(&Status::Deleted)?.eq("Deleted"));
    Ok(())
  }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
  pub user_id: String,
  pub prefix: Option<String>,
  pub email: String,
  pub status: Status,
  pub date_created: chrono::DateTime<chrono::Utc>,
}

impl Account {
  pub fn new(user_id: String, prefix: Option<String>, email: String, status: Status, date_created: chrono::DateTime<chrono::Utc>) -> Result<Account, Box<dyn Error>> {
    Ok( Account { user_id, prefix, email, status, date_created } )
  }

  fn new_from_identity(identity: &Identity) -> Result<Account, Box<dyn Error>> {
    Ok(Account {
         user_id: identity.id.clone(),
         email: identity.email.as_ref().ok_or_else(|| SimpleError::new("Unable to retrieve email"))?.clone(),
         prefix: None,
         status: Status::Active,
         date_created: chrono::Utc::now(),
    })
  }
}

#[cfg(test)]
impl Account {
  pub fn make_active_account() -> Account {
    Account {
      user_id: "ActiveUser".to_string(),
      prefix: Some("active".to_string()),
      email: "active@user.com".to_string(),
      status: Status::Active,
      date_created: chrono::prelude::Utc.ymd(2022, 1, 15).and_hms(8, 0, 0)
    }
  }

  pub fn make_deleted_account() -> Account {
    Account {
      user_id: "DeletedUser".to_string(),
      prefix: Some("deleted".to_string()),
      email: "deleted@user.com".to_string(),
      status: Status::Deleted,
      date_created: chrono::prelude::Utc.ymd(2022, 1, 15).and_hms(8, 0, 0)
    }
  }

  pub fn update(&mut self, account: Account) {
    self.user_id      = account.user_id;
    self.prefix       = account.prefix;
    self.email        = account.email;
    self.status       = account.status;
    self.date_created = account.date_created;
  }
}

#[async_trait]
pub trait Store {
  async fn get_account_from_user_id(&self, user_id: &str) -> Result<Option<Account>, Box<dyn Error>>;
  async fn get_account_from_prefix(&self, prefix: &str) -> Result<Option<Account>, Box<dyn Error>>;
  async fn put_account(&self, account: Account) -> Result<Account, Box<dyn Error>>;
  async fn update_account(&self, account: Account) -> Result<Account, Box<dyn Error>>;
  async fn delete_account(&self, user_id: &str) -> Result<Account, Box<dyn Error>>;
}

impl Account {
  pub async fn update_from_identity<T>(identity: &Identity, store: &T, update: HashMap<String, String>) -> Result<Account, Box<dyn Error>>
  where T: Store {
    let mut account = Account::from_identity(identity, store).await?;
    let mut updated = false;
    if let Some(prefix) = update.get("prefix") {
      if regex::Regex::new(r"^[a-zA-Z0-9]+$")?.is_match(prefix) {
        if store.get_account_from_prefix(prefix).await?.is_none() {
          account.prefix = Some(prefix.to_lowercase());
          updated = true;
        } else {
          return Err(Box::new(SimpleError::new(format!("Prefix {} is already taken.", prefix).as_str())));
        }
      } else {
        return Err(Box::new(SimpleError::new(format!("Prefix {} is invalid. prefix can only contains letters and numbers.", prefix).as_str())));
      }
    }

    if updated {
      store.update_account(account).await
    } else {
      Ok(account)
    }
  }
}

impl Account {
  pub async fn from_prefix<T>(prefix: &str, store: &T) -> Result<Option<Account>, Box<dyn Error>>
  where T: Store {
    store.get_account_from_prefix(prefix).await
  }

  pub async fn from_identity<T>(identity: &Identity, store: &T) -> Result<Account, Box<dyn Error>>
  where T: Store {
    log::info!("Make account from identity {:?}", identity);
    if identity.email.is_none() || (! identity.email_verified.unwrap_or_else(|| false)) {
      return Err(Box::new(SimpleError::new("Unable to make account from identity: email is not available or not verified")));
    }

    let mut account = store.get_account_from_user_id(identity.id.as_str()).await?;
    log::info!("Account retrieved from store: {:?}", account);
    if let None = account {
      log::info!("No account retrieved. Initialize a new one");
      let a = Account::new_from_identity(identity)?;
      log::info!("New account: {:?}", a);
      account = Some(store.put_account(a).await?);
    }
    log::info!("Account to be returned: {:?}", account);
    Ok(account.ok_or_else(|| SimpleError::new("No account were found and we were not able to initialise an account"))?)
  }
}

#[cfg(test)]
mod account {
  use super::*;
  use std::sync::Mutex;
  use std::cell::RefCell;
  use simple_error::SimpleError;
  use derive_builder::Builder;
  use crate::IdentityBuilder;

  #[derive(Default, Builder)]
  #[builder(pattern = "owned")]
  #[builder(setter(prefix = "set"))]
  struct StoreMock {
    #[builder(default)]
    pub accounts: Mutex<Vec<Account>>,
  }

  #[async_trait]
  impl Store for StoreMock {
    async fn get_account_from_user_id(&self, user_id: &str) -> Result<Option<Account>, Box<dyn Error>> {
      Ok(self.accounts
             .lock().map_err(|e| format!("{}", e))?
             .iter()
             .find(|a| a.user_id.eq(user_id) )
             .map(|a| a.clone())
        )
    }

    async fn get_account_from_prefix(&self, prefix: &str) -> Result<Option<Account>, Box<dyn Error>> {
      Ok(self.accounts
             .lock().map_err(|e| format!("{}", e))?
             .iter()
             .find(|a| { if let Some(p) = &a.prefix { p.eq(prefix) } else { false } })
             .map(|a| a.clone())
        )
    }

    async fn put_account(&self, account: Account) -> Result<Account, Box<dyn Error>> {
      self.accounts.lock().map_err(|e| format!("{}", e))?.push(account.clone());
      Ok(account)
    }

    async fn update_account(&self, account: Account) -> Result<Account, Box<dyn Error>> {
      let mut accounts = self.accounts.lock().map_err(|e| format!("{}", e))?;
      let a: &mut Account  = accounts
          .iter_mut()
          .find(|a| a.user_id.eq(account.user_id.as_str()))
          .ok_or_else(|| format!("Unable to find account with user_id {}", account.user_id))?;
      a.update(account);
      Ok(a.clone())
    }

    async fn delete_account(&self, user_id: &str) -> Result<Account, Box<dyn Error>> {
      let mut accounts = self.accounts.lock().map_err(|e| format!("{}", e))?;
      let mut account = accounts
          .iter_mut()
          .find(|a| a.user_id.eq(user_id))
          .ok_or_else(|| format!("Unable to find account with user_id {}", user_id))?;
      account.status = Status::Deleted;
      Ok(account.clone())
    }
  }
  
  #[tokio::test]
  async fn it_handles_request_with_store() -> Result<(), Box<dyn Error>> {
    let active_account = Account::make_active_account(); let deleted_account = Account::make_deleted_account();
    let store = StoreMockBuilder::default()
                .set_accounts(Mutex::new(vec![active_account.clone(), deleted_account.clone()]))
                .build()?;
    let a = Account::from_prefix("None", &store).await?;
    assert!(a.is_none());
    let a = Account::from_prefix(active_account.prefix.as_ref().unwrap().as_str(), &store).await?
            .ok_or_else(|| "Unable to retrieve active account from store")?;
    assert!(a.user_id.eq(active_account.user_id.as_str()));

    let identity = IdentityBuilder::default()
           .set_email(Some("value".to_string()))
           .set_email_verified(Some(true))
           .set_id(active_account.user_id.clone())
           .build()?;
    let a = Account::from_identity(&identity, &store).await?;
    assert!(a.email.eq(active_account.email.as_str()));

    let identity = IdentityBuilder::default()
           .set_email(Some("user1@user.com".to_string()))
           .set_email_verified(Some(true))
           .set_id("user1_id".to_string())
           .set_username("user1_username".to_string())
           .build()?;
    assert!(store.accounts.lock().unwrap().len() == 2);
    let a = Account::from_identity(&identity, &store).await?;
    assert!(store.accounts.lock().unwrap().len() == 3);
    assert!(a.user_id.eq(identity.id.as_str()));
    assert!(a.prefix.is_none());

    let a = Account::update_from_identity(&identity, &store,
        HashMap::from([ ("prefix".to_string(), "abc".to_string()) ])
      ).await?; 
    assert!(a.prefix.unwrap().eq("abc"));

    let a = Account::update_from_identity(&identity, &store,
        HashMap::from([ ("prefix".to_string(), active_account.prefix.unwrap().clone()) ])
      ).await; 
    assert!(a.is_err());

    Ok(())
  }
}

