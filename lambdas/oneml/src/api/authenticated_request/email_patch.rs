use super::*;

#[derive(serde::Deserialize)]
pub struct Params {
  status: constructs::email::Status,
}

pub async fn implementation<T>(store: &T, identity: Identity, email: &str, body: Params) -> Result<lambda_http::Response<String>>
where T: traits::store::EmailStore,
{
  log::info!("[PATCH] ApiEmail");
  let result = constructs::email::Email::update_from_identity_status(
    &identity, store, email, body.status).await?;
  Ok(lambda_http::Response::builder()
     .status(lambda_http::http::StatusCode::OK)
     .body(serde_json::to_string(&result)?)?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn it_patch_email() -> Result<()> {
    use traits::store::EmailStore;
    let store = traits::store::email::mock::EmailStoreMockBuilder::default().build().unwrap();
    let _ = store.save_email(
      constructs::email::Email::from_email_userid_status("one@u1.two.com", "u1", constructs::email::Status::Forward)
    ).await?;

    let r = implementation(&store,
      Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true),
      "one@u1.two.com", Params { status: constructs::email::Status::Block, },
    ).await?;
    assert!(matches!(r.status(), lambda_http::http::StatusCode::OK));
    let a = store.from_address("one@u1.two.com").await?.unwrap();
    assert!(matches!(a.status, constructs::email::Status::Block));

    assert!(implementation(&store,
      Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true),
      "two@u1.two.com", Params { status: constructs::email::Status::Forward, },
    ).await.is_err());
      
    Ok(())
  }


}
