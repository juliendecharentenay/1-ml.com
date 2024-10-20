use super::*;

pub enum AuthenticatedRequest {
  GetMe,
  PatchMe(std::collections::HashMap<String,String>),
  GetEmail,
  PatchEmail,
}

impl AuthenticatedRequest {
  pub async fn handle(self, identity: Identity) -> Result<lambda_http::Response<String>> {
    match self {
      AuthenticatedRequest::GetMe        => AuthenticatedRequest::get_me_impl(&aws::Store::default().await?, identity).await,
      AuthenticatedRequest::PatchMe(update) => AuthenticatedRequest::patch_me_impl(&aws::Store::default().await?, identity, update).await,
      AuthenticatedRequest::GetEmail     => AuthenticatedRequest::get_email_impl(&aws::Store::default().await?, identity).await,
      _ => Err(Error::NotImplementedYet)
    }
  }
}

impl AuthenticatedRequest {
  async fn patch_me_impl<T>(store: &T, identity: Identity, update: std::collections::HashMap<String, String>) -> Result<lambda_http::Response<String>>
  where T: traits::store::AccountStore,
  {
    log::info!("[PATCH] ApiMe");
    let result = constructs::Account::update_from_identity(&identity, store, update).await?;
    Ok(lambda_http::Response::builder()
      .status(lambda_http::http::StatusCode::OK)
      .body(serde_json::to_string(&result)?)?)
  }
}

#[cfg(test)]
mod patch_me {
  use super::*;

  #[tokio::test]
  async fn it_patches_me() -> Result<()> {
    use chrono::TimeZone;
    use traits::store::AccountStore;
    let store = traits::store::account::mock::AccountStoreMockBuilder::default().build().unwrap();
    let _ = store.put_account(constructs::Account::from_userid_prefix_email_status_datecreated(
      "u1", "prefix_u1", "one@home.com", constructs::account::Status::Active, 
      chrono::prelude::Utc.with_ymd_and_hms(2022, 1, 15, 8, 0, 0).unwrap())).await?;

    assert!(store.get_account_from_prefix("prefix_u2").await?.is_none());
    let _ = AuthenticatedRequest::patch_me_impl(&store,
      Identity::from_id_username_email_emailverified("u2", "u2 name", "two@home.com", true),
      std::collections::HashMap::from([
        ("prefix".to_string(), "prefix_u2".to_string())
      ])
    ).await?;
    assert!(store.get_account_from_prefix("prefix_u2").await?.is_some());
    
    // Invalid character
    let r = AuthenticatedRequest::patch_me_impl(&store,
      Identity::from_id_username_email_emailverified("u3", "u3 name", "three@home.com", true),
      std::collections::HashMap::from([
        ("prefix".to_string(), "Invalid$".to_string())
      ])
    ).await;
    assert!(r.is_err());

    // Prefix already taken
    let r = AuthenticatedRequest::patch_me_impl(&store,
      Identity::from_id_username_email_emailverified("u4", "u4 name", "four@home.com", true),
      std::collections::HashMap::from([
        ("prefix".to_string(), "prefix_U1".to_string())
      ])
    ).await;
    assert!(r.is_err());

    
    Ok(())
  }
}

impl AuthenticatedRequest {
  async fn get_me_impl<T>(store: &T, identity: Identity) -> Result<lambda_http::Response<String>>
  where T: traits::store::AccountStore,
  {
    log::info!("[GET] ApiMe: load account");
    let result = constructs::Account::from_identity(&identity, store).await?;
    Ok(lambda_http::Response::builder()
      .status(lambda_http::http::StatusCode::OK)
      .body(serde_json::to_string(&result)?)?)
  }
}

#[cfg(test)]
mod get_me {
  use super::*;

  #[tokio::test]
  async fn it_gets_me() -> Result<()> {
    use chrono::TimeZone;
    use traits::store::AccountStore;
    let store = traits::store::account::mock::AccountStoreMockBuilder::default().build().unwrap();
    let _ = store.put_account(constructs::Account::from_userid_prefix_email_status_datecreated(
      "u1", "prefix_u1", "one@home.com", constructs::account::Status::Active, 
      chrono::prelude::Utc.with_ymd_and_hms(2022, 1, 15, 8, 0, 0).unwrap())).await?;
    
    let r = AuthenticatedRequest::get_me_impl(&store, 
      Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true)
    ).await?;
    assert!(matches!(r.status(), lambda_http::http::StatusCode::OK));
    let r: constructs::Account = serde_json::from_str(r.body().as_str())?;
    assert!(r.user_id.eq("u1"));
    assert!(r.prefix.unwrap().eq("prefix_u1"));

    // Retrieving an unknown account creates it
    assert!(store.get_account_from_user_id("noone").await?.is_none());
    assert!(AuthenticatedRequest::get_me_impl(&store, 
      Identity::from_id_username_email_emailverified("noone", "noone name", "noone@home.com", true)
    ).await.is_ok());
    assert!(store.get_account_from_user_id("noone").await?.is_some());

    Ok(())
  }
}

impl AuthenticatedRequest {
  async fn get_email_impl<T>(store: &T, identity: Identity) -> Result<lambda_http::Response<String>> 
  where T: traits::store::EmailStore,
  {
    log::info!("[GET] ApiEmail: Load emails");
    let email_list = constructs::Email::list_from_identity(&identity, store).await?;
    Ok(lambda_http::Response::builder().status(lambda_http::http::StatusCode::OK).body(serde_json::to_string(&email_list)?)?)
  }
}

#[cfg(test)]
mod get_email {
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
