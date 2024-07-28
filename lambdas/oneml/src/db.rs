use super::*;

pub mod tables;

pub fn connection() -> error::Result<rusqlite::Connection> {
  let file = std::env::var("SQLITE_DB_FILE")?;
  Ok(rusqlite::Connection::open(file)?)
}


