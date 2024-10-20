use super::*;

pub enum AuthenticatedRequest {
  GetMe,
  PatchMe,
  GetEmail,
  PatchEmail,
}

impl AuthenticatedRequest {
  pub async fn handle(self, identity: Identity) -> Result<lambda_http::Response<String>> {
    match self {
      AuthenticatedRequest::GetEmail => self.get_email_impl(identity).await,
      _ => Err(Error::NotImplementedYet)
    }
  }
}

impl AuthenticatedRequest {
  async fn get_email_impl(&self, identity: Identity) -> Result<lambda_http::Response<String>> {
    log::info!("ApiEmailGet: Load emails");
    let store = aws::Store::default().await?;
    let email_list = email::Email::list_from_identity(&identity, &store).await?;
    Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&email_list)?)?)
  }
}
