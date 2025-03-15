use super::*;

pub struct Config { }
impl Config {
  pub fn user_table_name() -> Result<String>   { Ok(std::env::var("DYNAMODB_USERS")?) }
  pub fn prefix_table_name() -> Result<String> { Ok(std::env::var("DYNAMODB_PREFIX")?) }
  pub fn email_table_name() -> Result<String>  { Ok(std::env::var("DYNAMODB_EMAIL")?) }
}

