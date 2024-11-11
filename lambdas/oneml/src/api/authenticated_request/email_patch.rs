use super::*;

#[derive(serde::Deserialize)]
pub struct Params {
  status: constructs::email::Status,
}

pub async fn implementation<T>(store: &T, identity: Identity, email: &str, body: Params) -> Result<constructs::Email>
where T: traits::store::EmailStore,
{
  log::info!("[PATCH] ApiEmail");
  let mut e = store.from_address(email).await?.ok_or(Error::EmailNotFound(email.to_string()))?;
  if ! e.user_id.eq(identity.id.as_str()) {
    return Err(Error::InvalidEmailAccount(e.user_id));
  }
  e.status = body.status;
  store.update_email(e.clone()).await?;
  Ok(e)
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
    let a = store.from_address("one@u1.two.com").await?.unwrap();
    assert!(matches!(a.status, constructs::email::Status::Block));

    assert!(implementation(&store,
      Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true),
      "two@u1.two.com", Params { status: constructs::email::Status::Forward, },
    ).await.is_err());
      
    assert!(implementation(&store,
      Identity::from_id_username_email_emailverified("u2", "u2 name", "two@home.com", true),
      "one@u1.two.com", Params { status: constructs::email::Status::Forward, },
    ).await.is_err());
      
    Ok(())
  }
}
