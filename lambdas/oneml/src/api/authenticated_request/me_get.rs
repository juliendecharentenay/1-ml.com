use super::*;

pub async fn implementation<T>(store: &T, identity: Identity) -> Result<lambda_http::Response<String>>
where T: traits::store::AccountStore,
{
    log::info!("[GET] ApiMe: load account");
    let result = constructs::Account::from_identity(&identity, store).await?;
    Ok(lambda_http::Response::builder()
      .status(lambda_http::http::StatusCode::OK)
      .body(serde_json::to_string(&result)?)?)
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
    
    let r = implementation(&store, 
      Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true)
    ).await?;
    assert!(matches!(r.status(), lambda_http::http::StatusCode::OK));
    let r: constructs::Account = serde_json::from_str(r.body().as_str())?;
    assert!(r.user_id.eq("u1"));
    assert!(r.prefix.unwrap().eq("prefix_u1"));

    // Retrieving an unknown account creates it
    assert!(store.get_account_from_user_id("noone").await?.is_none());
    assert!(implementation(&store, 
      Identity::from_id_username_email_emailverified("noone", "noone name", "noone@home.com", true)
    ).await.is_ok());
    assert!(store.get_account_from_user_id("noone").await?.is_some());

    Ok(())
  }
}

