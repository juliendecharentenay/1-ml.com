use super::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Email account `{0}` does not match identity account")]
  InvalidEmailAccount(String),
  #[error("Email address `{0}` does not exist")]
  EmailNotFound(String),
  #[error("Empty request text body")]
  EmptyRequestTextBody,
  #[error(transparent)]
  FromUtf8Error(#[from] std::string::FromUtf8Error),
  #[error(transparent)]
  IdentityBuilderError(#[from] IdentityBuilderError),
  #[error(transparent)]
  ChronoParseError(#[from] chrono::ParseError),
  #[error(transparent)]
  Regex(#[from] regex::Error),
  #[error("AWS error: `{error}`")]
  AwsError { error: String },
  #[error("Unable to convert field `{field}`")]
  DynamodbConversion { field: &'static str },
  #[error(transparent)]
  LambdaHttpError(#[from] lambda_http::http::Error),
  #[error(transparent)]
  SerdeJsonError(#[from] serde_json::Error),
  #[error(transparent)]
  VarError(#[from] std::env::VarError),
  #[error(transparent)]
  RusqliteError(#[from] rusqlite::Error),
  #[error("Miscelleanous error: `{msg:?}`")]
  Misc { msg: String, },
  #[error("Not implemented yet")]
  NotImplementedYet,
}

impl Error {
  pub fn map_aws<T: std::fmt::Debug>(error: T) -> Error {
    Error::AwsError { error: format!("{error:?}") }
  }
}

impl Error {
  pub fn from<E>(e: E) -> Error
  where E: std::fmt::Debug
  {
    Error::Misc { msg: format!("{e:?}"), }
  }
}

impl std::convert::From<String> for Error {
  fn from(v: String) -> Error {
    Error::Misc { msg: v, }
  }
}

impl std::convert::From<&str> for Error {
  fn from(v: &str) -> Error {
    Error::Misc { msg: v.to_string(), }
  }
}
