use super::*;

pub struct Config {}
impl Config {
  pub fn bucket() -> Result<String, Error> { Ok(std::env::var("BUCKET_EMAILS")?) }
  pub fn ses_identity() -> Result<String, Error> { Ok(std::env::var("SES_IDENTITY")?) }
}

