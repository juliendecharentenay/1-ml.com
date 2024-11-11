use super::*;

pub async fn implementation<T>(store: &T, identity: Identity, update: std::collections::HashMap<String, String>) -> Result<constructs::Account>
where T: traits::store::AccountStore,
{
    log::info!("[PATCH] ApiMe");
    let result = constructs::Account::update_from_identity(&identity, store, update).await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
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
    let _ = implementation(&store,
      Identity::from_id_username_email_emailverified("u2", "u2 name", "two@home.com", true),
      std::collections::HashMap::from([
        ("prefix".to_string(), "prefix_u2".to_string())
      ])
    ).await?;
    assert!(store.get_account_from_prefix("prefix_u2").await?.is_some());
    
    // Invalid character
    assert!(implementation(&store,
      Identity::from_id_username_email_emailverified("u3", "u3 name", "three@home.com", true),
      std::collections::HashMap::from([
        ("prefix".to_string(), "Invalid$".to_string())
      ])
    ).await.is_err());

    // Prefix already taken
    assert!(implementation(&store,
      Identity::from_id_username_email_emailverified("u4", "u4 name", "four@home.com", true),
      std::collections::HashMap::from([
        ("prefix".to_string(), "prefix_U1".to_string())
      ])
    ).await.is_err());
    
    Ok(())
  }
}
