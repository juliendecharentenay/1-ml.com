use super::*;

#[async_trait::async_trait]
pub trait EmailStore {
  async fn email_list_from_user_id(&self, user_id: &str) -> Result<Vec<constructs::Email>>;
  async fn update_email(&self, email: constructs::Email) -> Result<constructs::Email>;
  async fn from_address(&self, email_address: &str) -> Result<Option<constructs::Email>>;
  async fn save_email(&self, email: constructs::Email) -> Result<constructs::Email>;
}

#[cfg(test)]
pub mod mock {
  use super::*;

  #[derive(Default, derive_builder::Builder)]
  #[builder(pattern = "owned")]
  #[builder(setter(prefix = "set"))]
  pub struct EmailStoreMock {
    #[builder(default)]
    pub emails: std::sync::Mutex<Vec<constructs::Email>>,
  }

  #[async_trait::async_trait]
  impl EmailStore for EmailStoreMock {
    async fn email_list_from_user_id(&self, user_id: &str) -> Result<Vec<constructs::Email>> {
      Ok(self.emails.lock().map_err(|e| format!("{e}"))?
             .iter()
             .filter(|e| e.user_id.eq(user_id))
             .cloned()
             .collect())
    }

    async fn update_email(&self, email: constructs::Email) -> Result<constructs::Email>{
      let mut emails = self.emails.lock().map_err(|e| format!("{e}"))?;
      let e: &mut constructs::Email = emails
        .iter_mut()
        .find(|e| e.user_id.eq(email.user_id.as_str()) && e.email.eq(email.email.as_str()))
        .ok_or_else(|| format!("Unable to find email {email} for user_id {user_id}", email = email.email, user_id = email.user_id))?;
      e.status = email.status;
      Ok(e.clone())
    }

    async fn from_address(&self, email_address: &str) -> Result<Option<constructs::Email>>{
      Ok(self.emails.lock().map_err(|e| format!("{e}"))?
             .iter()
             .find(|e| e.email.eq(email_address))
             .cloned())
    }

    async fn save_email(&self, email: constructs::Email) -> Result<constructs::Email>{
      self.emails.lock().map_err(|e| format!("{e}"))?.push(email.clone());
      Ok(email)
    }
  }
}

