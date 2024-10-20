use super::*;

#[derive(Debug, Default, derive_builder::Builder)]
#[builder(pattern = "owned")]
#[builder(setter(prefix = "set"))]
pub struct Identity {
  #[builder(default)]
  pub id: String,
  #[builder(default)]
  pub username: String,
  #[builder(default)]
  pub email: Option<String>,
  #[builder(default)]
  pub email_verified: Option<bool>,
}

impl Identity {
  pub fn from_id_username_email_emailverified(id: &str, username: &str, email: &str, email_verified: bool) -> Identity {
    Identity { id: id.to_string(), username: username.to_string(), email: Some(email.to_string()), email_verified: Some(email_verified) }
  }
  pub fn set_id(mut self, id: &str) -> Identity { self.id = id.to_string(); self }
  pub fn set_username(mut self, username: &str) -> Identity { self.username = username.to_string(); self }
  pub fn set_email(mut self, email: &str) -> Identity { self.email = Some(email.to_string()); self }
  pub fn set_email_verified(mut self, email_verified: bool) -> Identity { self.email_verified = Some(email_verified); self }
}

impl Identity {
  pub fn from_authorizer(value: &serde_json::Value) -> Result<Identity> {
    if ! value.is_object() { return Err(format!("Unable to retrieve valid identity from {:?}", value).as_str().into()); }
    let value = value.as_object().ok_or("Unable to retrieve object")?;

    let token_use = value.get("token_use").ok_or("Unable to retrieve token use")?;
    if ! token_use.is_string() { return Err("Unable to retrieve token use value".into()); }
    if token_use.as_str().ok_or("Unable to convert token use to string")? != "id" {
      return Err(format!("Invalid token use value {}", token_use.as_str().unwrap()).as_str().into());
    }

    let id = value.get("sub").ok_or("Unable to retrieve id")?;
    let id = id.as_str().ok_or("Unable to convert id to string")?.to_string();

    let username = value.get("cognito:username").ok_or("Unable to retrieve username")?;
    let username = username.as_str().ok_or("Unable to convert username to string")?.to_string();

    let email = match value.get("email") {
      Some(v) => {
        if v.is_string() {
          Some(v.as_str().ok_or("Unable to convert email to string")?.to_string())
        } else {
          None
        }
      },
      None => None,
    };

    let email_verified = match value.get("email_verified") {
      Some(v) => {
        if v.is_string() {
          Some(v.as_str().ok_or("Unable to convert email_verified to string")? == "true")
        } else {
          None
        }
      },
      None => None,
    };

    Ok(Identity { id, username, email, email_verified })
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_authorizer_works() -> Result<()> {
        let data = r#"{
      "aud": "aud123", "auth_time": "1700000000", "cognito:username": "usname", "email": "us@me.com", "email_verified": "true", 
      "event_id": "evtid123", "exp": "Sat Jun 01 20:33:43 UTC 2022", "iat": "Sat Jun 01 19:33:43 UTC 2022", "iss": "https://cognito-idp.eu-west-1.amazonaws.com/eu-east-1_abc", "jti": "jti123", "origin_jti": "origin_jti123", "sub": "sub123", "token_use": "id"
}"#;
        let v: serde_json::Value = serde_json::from_str(data)?;
        let identity = Identity::from_authorizer(&v)?;
        assert!(identity.id.as_str() == "sub123");
        assert!(identity.username.as_str() == "usname");
        assert!(identity.email == Some("us@me.com".to_string()));
        assert!(identity.email_verified == Some(true));
        Ok(())
    }
}
