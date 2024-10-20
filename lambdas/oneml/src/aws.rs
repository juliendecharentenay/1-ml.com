use super::*;

use std::{
  collections::HashMap,
};
use async_trait::async_trait;
// use futures_util::StreamExt;

use crate::{account, email};

mod config;

pub struct Store { 
  client: aws_sdk_dynamodb::Client,
}

impl Store {
  pub async fn default() -> Result<Store> { 
    let shared_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_dynamodb::Client::new(&shared_config);
    Ok( Store { client } )
  }
}

impl Store {
    fn queries_to_account(user_item: HashMap<String, aws_sdk_dynamodb::types::AttributeValue>, 
                          prefix_item: Option<HashMap<String, aws_sdk_dynamodb::types::AttributeValue>>) -> Result<account::Account> {
      let user_id = user_item.get("UserId").ok_or("Retrieved item does not contain field UserId")?
                  .as_s().map_err(|_| Error::DynamodbConversion { field: "UserId" })?.to_string();
      let email   = user_item.get("Email").ok_or("Retrieved item does not contain field Email")?
                  .as_s().map_err(|_| Error::DynamodbConversion { field: "Email" })?.to_string();
      let date_created  = user_item.get("DateCreated").ok_or("Retrieved item does not contain field DateCreated")?
                  .as_s().map_err(|_| Error::DynamodbConversion { field: "DateCreated" })?
                  .parse::<chrono::DateTime<chrono::Utc>>()?;
      let status  = user_item.get("Status").ok_or("Retrieved item does not contain field Status")?
                  .as_s().map_err(|_| Error::DynamodbConversion { field: "Status" })?;
      let status  = account::Status::from_str(status)?;
      let prefix = match prefix_item {
          Some(item) => Some(item.get("Prefix").ok_or("Retrieved item does not contain field Prefix")?
              .as_s().map_err(|_| Error::DynamodbConversion { field: "Prefix" })?.to_string()),
          None => None,
      };
      Ok(account::Account::new(user_id, prefix, email, status, date_created)?)
    }
}

#[async_trait]
impl account::Store for Store {
  async fn get_account_from_user_id(&self, user_id: &str) -> Result<Option<account::Account>> {
    log::info!("Get account for user id: {}", user_id);
    let user_q = self.client.get_item()
            .table_name(config::Config::USER_TABLE_NAME)
            .key("UserId", aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()))
            .send().await
            .map_err(Error::map_aws)?;
    Store::log_cu(&user_q.consumed_capacity);
    if user_q.item.is_none() { return Ok(None); }

    log::info!("... and prefix:");
    let prefix_q = self.client.query()
        .table_name(config::Config::PREFIX_TABLE_NAME)
        .index_name("UserIdIndex") // Use global index
        .key_condition_expression("UserId = :u")
        .expression_attribute_values(":u", aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()))
        .send().await
        .map_err(Error::map_aws)?;
    Store::log_cu(&prefix_q.consumed_capacity);
    let prefix_q = match prefix_q.items {
      Some(r) => {
        if r.len() == 0 {
          None
        } else {
          r.into_iter().nth(0)
        }
      },
      None => None,
    };
    log::info!("...prefix_q = {:?}", prefix_q);

    Store::queries_to_account(user_q.item.ok_or("Unable to retrieve user item")?,
                             prefix_q)
      .map(|a| Some(a))
  }

  async fn is_prefix_used(&self, prefix: &str) -> Result<bool> {
    log::info!("Check for prefix {prefix} in database");
    let prefix_q = self.client.get_item()
        .table_name(config::Config::PREFIX_TABLE_NAME)
        .key("Prefix", aws_sdk_dynamodb::types::AttributeValue::S(prefix.to_string()))
        .send().await
        .map_err(Error::map_aws)?;
    Ok(prefix_q.item.is_some())
  }

  async fn get_account_from_prefix(&self, prefix: &str) -> Result<Option<account::Account>> {
    log::info!("Get account associated with prefix{:?}", prefix);
    let prefix_q = self.client.get_item()
        .table_name(config::Config::PREFIX_TABLE_NAME)
        .key("Prefix", aws_sdk_dynamodb::types::AttributeValue::S(prefix.to_string()))
        .send().await
        .map_err(Error::map_aws)?;
    Store::log_cu(&prefix_q.consumed_capacity);
    if prefix_q.item.is_none() { return Err(format!("Unable to retrieve information from Prefix {}", prefix).as_str().into()); }
    let prefix_item = prefix_q.item.as_ref().ok_or("Unable to retrieve prefix item")?;
    let user_id = prefix_item.get("UserId").ok_or("Retrieved item does not contain field UserId")?;

    let user_q = self.client.get_item()
        .table_name(config::Config::USER_TABLE_NAME)
        .key("UserId", user_id.clone())
        .send().await
        .map_err(Error::map_aws)?;
    Store::log_cu(&user_q.consumed_capacity);
    if user_q.item.is_none() { return Err(format!("Unable to retrieve user from UserId {:?}", user_id).as_str().into()); }

    Store::queries_to_account(user_q.item.ok_or("Unable to retrieve user item")?,
                             prefix_q.item)
      .map(|a| Some(a))
  }

  async fn put_account(&self, account: account::Account) -> Result<account::Account> {
    log::info!("Store account {:?}", account);
    let status = account::Status::to_str(&account.status)?;
    let r = self.client.put_item()
      .table_name(config::Config::USER_TABLE_NAME)
      .condition_expression("attribute_not_exists(UserId)")
      .item("UserId", aws_sdk_dynamodb::types::AttributeValue::S(account.user_id.clone()))
      .item("Email",  aws_sdk_dynamodb::types::AttributeValue::S(account.email.clone()))
      .item("Status", aws_sdk_dynamodb::types::AttributeValue::S(status))
      .item("DateCreated", aws_sdk_dynamodb::types::AttributeValue::S(account.date_created.to_string()))
      .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
      .send().await
      .map_err(Error::map_aws)?;
    if r.attributes.is_some() {
      return Err(format!("Creation of new account seems to have over-written another account {:#?}", r.attributes).as_str().into());
    }
    Ok(account)
  }

  async fn update_account(&self, account: account::Account) -> Result<account::Account> {
    log::info!("Update account {:?}", account);
    let status = account::Status::to_str(&account.status)?;
    let _user_q = self.client.put_item()
         .table_name(config::Config::USER_TABLE_NAME)
         .condition_expression("attribute_exists(UserId)")
         .item("UserId", aws_sdk_dynamodb::types::AttributeValue::S(account.user_id.clone()))
         .item("Email", aws_sdk_dynamodb::types::AttributeValue::S(account.email.clone()))
         .item("Status", aws_sdk_dynamodb::types::AttributeValue::S(status))
         .item("DateCreated", aws_sdk_dynamodb::types::AttributeValue::S(account.date_created.to_string()))
         .return_values(aws_sdk_dynamodb::types::ReturnValue::None)
         .send().await
         .map_err(Error::map_aws)?;
     if let Some(prefix) = account.prefix.as_ref() {
       let _prefix_q = self.client.put_item()
         .table_name(config::Config::PREFIX_TABLE_NAME)
         .condition_expression("( attribute_not_exists(UserId) AND attribute_not_exists(Prefix) ) OR ( UserId = :u AND Prefix = :p )")
         .expression_attribute_values(":u", aws_sdk_dynamodb::types::AttributeValue::S(account.user_id.clone()))
         .expression_attribute_values(":p", aws_sdk_dynamodb::types::AttributeValue::S(prefix.clone()))
         .item("UserId", aws_sdk_dynamodb::types::AttributeValue::S(account.user_id.clone()))
         .item("Prefix", aws_sdk_dynamodb::types::AttributeValue::S(prefix.clone()))
         .send().await
         .map_err(Error::map_aws)?;
     }
     Ok(account)
  }
/*
  async fn update_account(&self, user_id: &str, update: HashMap<String, String>) -> Result<()> {
    log::info!("Update account for user id {} with properties {:?}", user_id, update);
    if let Some(v) = update.get("prefix") {
      log::info!("Update prefix");
      log::info!("Check if user already has a prefix assigned");
      let prefix_check = self.client.get_item()
          .table_name(config::Config::PREFIX_TABLE_NAME)
          .key("UserId", aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()))
          .send().await?;
      Store::log_cu(&prefix_check.consumed_capacity);
      if prefix_check.item.is_some() {
        return Err(Box::new(SimpleError::new("User id already has a prefix assigned")));
      }

      log::info!("Check that prefix {} is not already taken", v);
      let prefix_check = self.client.get_item()
          .table_name(config::Config::PREFIX_TABLE_NAME)
          .key("Prefix", aws_sdk_dynamodb::types::AttributeValue::S(v.clone()))
          .send().await?;
      Store::log_cu(&prefix_check.consumed_capacity);
      if prefix_check.item.is_some() {
        return Err(Box::new(SimpleError::new(format!("Prefix {} is already taken", v).as_str())));
      }

      // Update item
      let _r = self.client.put_item()
          .table_name(config::Config::PREFIX_TABLE_NAME)
          .condition_expression("attribute_not_exists(UserId) AND attribute_not_exists(Prefix)")
          .item("UserId", aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()))
          .item("Prefix", aws_sdk_dynamodb::types::AttributeValue::S(v.clone()))
          .send().await?;
    }
    Ok(())
  }
*/

  async fn delete_account(&self, user_id: &str) -> Result<account::Account> {
    log::info!("Delete account associated with user id {}", user_id);
    let user_query = self.client.delete_item()
          .table_name(config::Config::USER_TABLE_NAME)
          .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
          .key("UserId", aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()))
          .send().await
          .map_err(Error::map_aws)?;
    if user_query.attributes.is_none() {
      return Err(format!("Account associated with user id {} does not exist", user_id).as_str().into());
    }

    let prefix_query = self.client.update_item()
          .table_name(config::Config::PREFIX_TABLE_NAME)
          .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
          .condition_expression("attribute_exists(UserId)")
          .key("UserId", aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()))
          .update_expression("SET UserId = :u")
          .expression_attribute_values(":u", aws_sdk_dynamodb::types::AttributeValue::S("-1".to_string()))
          .send().await
          .map_err(Error::map_aws)?;

     Store::queries_to_account(user_query.attributes.ok_or("Unable to retrieve user item")?,
                 prefix_query.attributes)
  }
}

impl Store {
  fn query_to_email(item: HashMap<String, aws_sdk_dynamodb::types::AttributeValue>) -> Result<email::Email> {
    let user_id = item.get("UserId").ok_or("Retrieved item does not contain field UserId")?
                  .as_s().map_err(|_| Error::DynamodbConversion { field: "UserId" })?.to_string();
    let email   = item.get("Email").ok_or("Retrieved item does not contain field Email")?
                  .as_s().map_err(|_| Error::DynamodbConversion { field: "Email" })?.to_string();
    let status  = item.get("Status").ok_or("Retrieved item does not contain field Status")?
                  .as_s().map_err(|_| Error::DynamodbConversion { field: "Status" })?;
    let status  = email::Status::from_str(status)?;
    Ok( email::Email::new(email, user_id)?.status(status)? )
  }
}

#[async_trait]
impl email::Store for Store {
  async fn save_email(&self, email: email::Email) -> Result<email::Email> {
    log::info!("Save email");
    let status = email::Status::to_str(&email.status)?;
    let _email_q = self.client.put_item()
        .table_name(config::Config::EMAIL_TABLE_NAME)
        .condition_expression("attribute_not_exists(Email)")
        .item("UserId", aws_sdk_dynamodb::types::AttributeValue::S(email.user_id.clone()))
        .item("Email",  aws_sdk_dynamodb::types::AttributeValue::S(email.email.clone()))
        .item("Status", aws_sdk_dynamodb::types::AttributeValue::S(status))
        .send().await.map_err(Error::map_aws)?;
    Ok(email)
  }

  async fn email_list_from_user_id(&self, user_id: &str) -> Result<Vec<email::Email>> {
    log::info!("List emails for user {}", user_id);
    let mut email_q = self.client.query()
       .table_name(config::Config::EMAIL_TABLE_NAME)
       .index_name("UserIdIndex") // Global index
       .key_condition_expression("UserId = :u")
       .expression_attribute_values(":u", aws_sdk_dynamodb::types::AttributeValue::S(user_id.to_string()))
       .into_paginator()
       .send();

    let mut email_list = Vec::new();
    while let Some(page) = email_q.next().await {
      let query_output = page.map_err(Error::map_aws)?;
      if let Some(items) = query_output.items {
        for item in items.into_iter() {
          email_list.push(Store::query_to_email(item)?);
        }
      }
    }

    Ok(email_list)
  }

  async fn update_email(&self, email: email::Email) -> Result<email::Email> {
    log::info!("Update email");
    let status = email::Status::to_str(&email.status)?;
    let _email_q = self.client.put_item()
        .table_name(config::Config::EMAIL_TABLE_NAME)
        .condition_expression("attribute_exists(Email) AND attribute_exists(UserId)")
        .item("UserId", aws_sdk_dynamodb::types::AttributeValue::S(email.user_id.clone()))
        .item("Email",  aws_sdk_dynamodb::types::AttributeValue::S(email.email.clone()))
        .item("Status", aws_sdk_dynamodb::types::AttributeValue::S(status))
        .send().await.map_err(Error::map_aws)?;
    Ok(email)
  }

  async fn from_address(&self,  email_address: &str) -> Result<Option<email::Email>> {
    let email_q = self.client.get_item()
        .table_name(config::Config::EMAIL_TABLE_NAME)
        .key("Email", aws_sdk_dynamodb::types::AttributeValue::S(email_address.to_string()))
        .send().await.map_err(Error::map_aws)?;
    match email_q.item {
      Some(item) => Ok(Some(Store::query_to_email(item)?)),
      None       => Ok(None),
    }
  }
}

impl Store {
  fn log_cu(c: &Option<aws_sdk_dynamodb::types::ConsumedCapacity>) {
    if let Some(c) = c.as_ref() { 
      if let Some(cu) = c.read_capacity_units {
        if cu > 0.0 { log::info!("... read consumption: {}RRU", cu); }
      }
      if let Some(cu) = c.write_capacity_units {
        if cu > 0.0 { log::info!("... write consumption: {}WRU", cu); }
      }
    }
  }
}

