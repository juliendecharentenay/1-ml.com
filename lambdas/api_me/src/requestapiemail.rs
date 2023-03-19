use async_trait::async_trait;
use lambda_http::RequestExt;

use crate::handler;

///
/// Handle [GET] /api/emails requests
///
#[derive(Default)]
pub struct Get {}

#[async_trait]
impl handler::Request for Get {
  async fn run_with_identity(&self, 
              _event: &lambda_http::Request,
              identity: oneml::Identity) -> Result<(http::StatusCode, String), Box<dyn std::error::Error>> {
    log::info!("ApiEmailGet: Load emails");
    let store = oneml::aws::Store::default().await?;
    let email_list = oneml::email::Email::list_from_identity(&identity, &store).await?;
    Ok((http::StatusCode::OK, serde_json::to_string(&email_list)?))
  }
}

///
/// Handle [PATCH] /api/emails/{email} requests
///
#[derive(Default)]
pub struct Patch {}

#[async_trait]
impl handler::Request for Patch {
  async fn run_with_identity_body(&self, 
              event: &lambda_http::Request,
              identity: oneml::Identity,
              body: &String) -> Result<(http::StatusCode, String), Box<dyn std::error::Error>> {
    log::info!("ApiEmailUdPatch: Update email");
    let email = event.path_parameters().first("email").map(urlencoding::decode).ok_or("Unable to retrieve email path parameter")??.to_string();
    let store = oneml::aws::Store::default().await?;
    let email = oneml::email::Email::update_from_identity(&identity, &store, email, serde_json::from_str(body)?).await?;
    Ok((http::StatusCode::OK, serde_json::to_string(&email)?))
  }
}
