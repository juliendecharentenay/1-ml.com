use async_trait::async_trait;

use crate::handler;

///
/// Handle [GET] /api/me
///
#[derive(Default)]
pub struct Get {}

#[async_trait]
impl handler::Request for Get {
  async fn run_with_identity(&self, 
              _event: &lambda_http::Request,
              identity: oneml::Identity) -> Result<(http::StatusCode, String), Box<dyn std::error::Error>> {
    log::info!("ApiMeGet:: Load account");
    let store = oneml::aws::Store::default().await?;
    let account = oneml::account::Account::from_identity(&identity, &store).await?;
    Ok((http::StatusCode::OK, serde_json::to_string(&account)?))
  }
}

///
/// Handle [PATCH] /api/me
///
#[derive(Default)]
pub struct Patch {}

#[async_trait]
impl handler::Request for Patch {
  async fn run_with_identity_body(&self, 
                _event: &lambda_http::Request, 
                identity: oneml::Identity,
                body: &String) -> Result<(http::StatusCode, String), Box<dyn std::error::Error>> {
    log::info!("ApiMePatch: Update account");
    let store = oneml::aws::Store::default().await?;
    let account = oneml::account::Account::update_from_identity(&identity, &store, serde_json::from_str(body)?).await?;
    Ok((http::StatusCode::OK, serde_json::to_string(&account)?))
  }
}

