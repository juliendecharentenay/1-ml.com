use super::*;
// use std::backtrace::Backtrace;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  VarError(#[from] std::env::VarError),
  #[error(transparent)]
  RusqliteError(#[from] rusqlite::Error),
  #[error("Miscelleanous error: `{msg:?}`")]
  Misc { msg: String, },
}

impl Error {
  pub fn from<E>(e: E) -> Error
  where E: std::fmt::Debug
  {
    Error::Misc { msg: format!("{e:?}"), }
  }
}

impl std::convert::From<&str> for Error {
  fn from(v: &str) -> Error {
    Error::Misc { msg: v.to_string(), }
  }
}
