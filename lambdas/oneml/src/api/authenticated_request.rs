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
      AuthenticatedRequest::GetEmail => AuthenticatedRequest::get_email_impl(&aws::Store::default().await?, identity).await,
      _ => Err(Error::NotImplementedYet)
    }
  }
}

impl AuthenticatedRequest {
  async fn get_email_impl<T>(store: &T, identity: Identity) -> Result<lambda_http::Response<String>> 
  where T: traits::store::EmailStore,
  {
    log::info!("ApiEmailGet: Load emails");
    let email_list = constructs::Email::list_from_identity(&identity, store).await?;
    Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&email_list)?)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn it_gets_email() -> Result<()> {
    use traits::store::EmailStore;
    let store = traits::store::email::mock::EmailStoreMockBuilder::default().build().unwrap();
    let _ = store.save_email(
      constructs::email::Email::from_email_userid_status("one@two.com", "u1", constructs::email::Status::Forward)
    ).await?;


    let r = AuthenticatedRequest::get_email_impl(&store, 
      Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true)
    ).await?;
    assert!(matches!(r.status(), lambda_http::http::StatusCode::OK));
    let r: Vec<constructs::Email> = serde_json::from_str(r.body().as_str())?;
    assert!(r.len() == 1);
    assert!(r[0].email.eq("one@two.com"));

    let r = AuthenticatedRequest::get_email_impl(&store,
      Identity::from_id_username_email_emailverified("u2", "u2 name", "two@home.com", true)
    ).await?;
    assert!(matches!(r.status(), lambda_http::http::StatusCode::OK));
    let r: Vec<constructs::Email> = serde_json::from_str(r.body().as_str())?;
    assert!(r.len() == 0);

    Ok(())
  }
}
