//[openapi]   authenticated_request:
use super::*;

mod me_get;
mod me_patch;
mod email_get;
mod email_patch;
mod received_email_get;

pub enum AuthenticatedRequest {
  GetMe,
  PatchMe(std::collections::HashMap<String,String>),
  GetEmail,
  PatchEmail { email: String, body: email_patch::Params, },
  GetReceivedEmail { email: String, },
}

impl AuthenticatedRequest {
  pub async fn handle(self, identity: Identity) -> Result<lambda_http::Response<String>> {
    match self {
      AuthenticatedRequest::GetMe           => to_response(&me_get::implementation(&aws::Store::default().await?, identity).await?),
      AuthenticatedRequest::PatchMe(update) => to_response(&me_patch::implementation(&aws::Store::default().await?, identity, update).await?),
      AuthenticatedRequest::GetEmail                    => to_response(&email_get::implementation(&aws::Store::default().await?, &mut db::connection()?, identity, chrono::Utc::now().naive_local()).await?),
      AuthenticatedRequest::PatchEmail { email, body }  => to_response(&email_patch::implementation(&aws::Store::default().await?, identity, email.as_str(), body).await?),
      AuthenticatedRequest::GetReceivedEmail { email } => to_response(&received_email_get::implementation(&mut db::connection()?, identity, email.as_str()).await?),
    }
  }
}

