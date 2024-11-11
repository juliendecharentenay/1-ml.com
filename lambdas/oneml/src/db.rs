use super::*;

pub mod tables;

pub fn connection() -> error::Result<rusqlite::Connection> {
  let file = std::env::var("SQLITE_DB_FILE")?;
  Ok(rusqlite::Connection::open(file)?)
}

#[cfg(test)]
pub fn db_in_memory() -> Result<rusqlite::Connection> {
  Ok(rusqlite::Connection::open_in_memory()?)
}

