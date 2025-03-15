pub type MyResult<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("mimetype `{ty}` is not supported")]
  UnsupportedMimetype { ty: String },
  #[error(transparent)]
  MailParseError(#[from] mailparse::MailParseError),
  #[error(transparent)]
  StdEnvVarError(#[from] std::env::VarError),
}
