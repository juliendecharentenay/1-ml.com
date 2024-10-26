use super::*;

mod me_get;
mod me_patch;
mod email_get;
mod email_patch;

pub enum AuthenticatedRequest {
  GetMe,
  PatchMe(std::collections::HashMap<String,String>),
  GetEmail,
  PatchEmail { email: String, body: email_patch::Params, },
}

impl AuthenticatedRequest {
  pub async fn handle(self, identity: Identity) -> Result<lambda_http::Response<String>> {
    match self {
      AuthenticatedRequest::GetMe        => me_get::implementation(&aws::Store::default().await?, identity).await,
      AuthenticatedRequest::PatchMe(update) => me_patch::implementation(&aws::Store::default().await?, identity, update).await,
      AuthenticatedRequest::GetEmail     => email_get::implementation(&aws::Store::default().await?, identity).await,
      AuthenticatedRequest::PatchEmail { email, body }  => email_patch::implementation(&aws::Store::default().await?, identity, email.as_str(), body).await,
    }
  }
}

