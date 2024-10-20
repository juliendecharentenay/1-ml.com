use super::*;

use std::{
  collections::HashMap,
};
use serde::{Serialize, Deserialize};
#[cfg(test)]
use chrono::TimeZone;

use crate::Identity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
  Active,
  Deleted,
}

impl Status {
  pub fn from_str(v: &str) -> Result<Status> {
    match v {
      "Active"  => Ok(Status::Active),
      "Deleted" => Ok(Status::Deleted),
      _         => Err(format!("{} is not a valid status", v).as_str().into()),
    }
  }

  pub fn to_str(status: &Status) -> Result<String> {
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
  fn str_conversion() -> Result<()> {
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
  pub fn new(user_id: String, prefix: Option<String>, email: String, status: Status, date_created: chrono::DateTime<chrono::Utc>) -> Result<Account> {
    Ok( Account { user_id, prefix, email, status, date_created } )
  }

  fn new_from_identity(identity: &Identity) -> Result<Account> {
    Ok(Account {
         user_id: identity.id.clone(),
         email: identity.email.as_ref().ok_or("Unable to retrieve email")?.clone(),
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
      date_created: chrono::prelude::Utc.with_ymd_and_hms(2022, 1, 15, 8, 0, 0).unwrap(),
    }
  }

  pub fn make_deleted_account() -> Account {
    Account {
      user_id: "DeletedUser".to_string(),
      prefix: Some("deleted".to_string()),
      email: "deleted@user.com".to_string(),
      status: Status::Deleted,
      date_created: chrono::prelude::Utc.with_ymd_and_hms(2022, 1, 15, 8, 0, 0).unwrap(),
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

impl Account {
  pub async fn update_from_identity<T>(identity: &Identity, store: &T, update: HashMap<String, String>) -> Result<Account>
  where T: traits::store::AccountStore {
    let mut account = Account::from_identity(identity, store).await?;
    let mut updated = false;
    if let Some(prefix) = update.get("prefix") {
      if regex::Regex::new(r"^[a-zA-Z0-9]+$")?.is_match(prefix) {
        if ! store.is_prefix_used(prefix).await? {
          account.prefix = Some(prefix.to_lowercase());
          updated = true;
        } else {
          return Err(format!("Prefix {} is already taken.", prefix).as_str().into());
        }
      } else {
        return Err(format!("Prefix {} is invalid. prefix can only contains letters and numbers.", prefix).as_str().into());
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
  pub async fn from_prefix<T>(prefix: &str, store: &T) -> Result<Option<Account>>
  where T: traits::store::AccountStore {
    store.get_account_from_prefix(prefix).await
  }

  pub async fn from_identity<T>(identity: &Identity, store: &T) -> Result<Account>
  where T: traits::store::AccountStore {
    log::info!("Make account from identity {:?}", identity);
    if identity.email.is_none() || (! identity.email_verified.unwrap_or_else(|| false)) {
      return Err("Unable to make account from identity: email is not available or not verified".into());
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
    Ok(account.ok_or("No account were found and we were not able to initialise an account")?)
  }
}

#[cfg(test)]
mod account {
  use super::*;
  use traits::store::account::mock::StoreMockBuilder;

  #[tokio::test]
  async fn it_handles_request_with_store() -> Result<()> {
    let active_account = Account::make_active_account(); let deleted_account = Account::make_deleted_account();
    let store = StoreMockBuilder::default()
                .set_accounts(std::sync::Mutex::new(vec![active_account.clone(), deleted_account.clone()]))
                .build().unwrap();
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

