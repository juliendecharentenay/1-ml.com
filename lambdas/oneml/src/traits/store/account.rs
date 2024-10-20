use super::*;

#[async_trait::async_trait]
pub trait AccountStore {
  async fn get_account_from_user_id(&self, user_id: &str) -> Result<Option<constructs::Account>>;
  async fn is_prefix_used(&self, prefix: &str) -> Result<bool>;
  async fn get_account_from_prefix(&self, prefix: &str) -> Result<Option<constructs::Account>>;
  async fn put_account(&self, account: constructs::Account) -> Result<constructs::Account>;
  async fn update_account(&self, account: constructs::Account) -> Result<constructs::Account>;
  async fn delete_account(&self, user_id: &str) -> Result<constructs::Account>;
}

#[cfg(test)]
pub mod mock {
  use super::*;

  use std::sync::Mutex;

  #[derive(Default, derive_builder::Builder)]
  #[builder(pattern = "owned")]
  #[builder(setter(prefix = "set"))]
  pub struct AccountStoreMock {
    #[builder(default)]
    pub accounts: Mutex<Vec<constructs::Account>>,
  }

  #[async_trait::async_trait]
  impl AccountStore for AccountStoreMock {
    async fn get_account_from_user_id(&self, user_id: &str) -> Result<Option<constructs::Account>> {
      Ok(self.accounts
             .lock().map_err(|e| format!("{}", e))?
             .iter()
             .find(|a| a.user_id.eq(user_id) )
             .map(|a| a.clone())
        )
    }

    async fn is_prefix_used(&self, prefix: &str) -> Result<bool> {
      self.get_account_from_prefix(prefix).await
      .map(|o| o.is_some())
    }

    async fn get_account_from_prefix(&self, prefix: &str) -> Result<Option<constructs::Account>> {
      Ok(self.accounts
             .lock().map_err(|e| format!("{}", e))?
             .iter()
             .find(|a| { if let Some(p) = &a.prefix { p.eq(prefix) } else { false } })
             .map(|a| a.clone())
        )
    }

    async fn put_account(&self, account: constructs::Account) -> Result<constructs::Account> {
      self.accounts.lock().map_err(|e| format!("{}", e))?.push(account.clone());
      Ok(account)
    }

    async fn update_account(&self, account: constructs::Account) -> Result<constructs::Account> {
      let mut accounts = self.accounts.lock().map_err(|e| format!("{}", e))?;
      let a: &mut constructs::Account  = accounts
          .iter_mut()
          .find(|a| a.user_id.eq(account.user_id.as_str()))
          .ok_or_else(|| format!("Unable to find account with user_id {}", account.user_id))?;
      a.update(account);
      Ok(a.clone())
    }

    async fn delete_account(&self, user_id: &str) -> Result<constructs::Account> {
      let mut accounts = self.accounts.lock().map_err(|e| format!("{}", e))?;
      let account = accounts
          .iter_mut()
          .find(|a| a.user_id.eq(user_id))
          .ok_or_else(|| format!("Unable to find account with user_id {}", user_id))?;
      account.status = constructs::account::Status::Deleted;
      Ok(account.clone())
    }
  }
}
  


