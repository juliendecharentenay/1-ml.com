use serde::{Serialize};

#[derive(Serialize)]
pub struct Unauthorized {
  pub message: String,
}

#[derive(Serialize)]
pub struct InternalServerError {
  pub message: String,
}

#[derive(Serialize)]
pub struct Ok {
  pub message: String,
}

impl Ok {
  pub fn default() -> Ok { Ok { message: "ok".to_string() } }
}

