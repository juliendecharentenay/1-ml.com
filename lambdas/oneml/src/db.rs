use super::*;

pub mod tables;

pub fn connection() -> error::Result<rusqlite::Connection> {
  let file = std::env::var("SQLITE_DB_FILE")?;
  Ok(rusqlite::Connection::open(file)?)
}

pub fn postgres_connection() -> Result<postgres::Client> {
  Ok(postgres::Client::configure()
    .user(std::env::var("POSTGRESQL_USER")?.as_str())
    .password(std::env::var("POSTGRESQL_PASSWORD")?)
    .dbname(std::env::var("POSTGRESQL_DBNAME")?.as_str())
    .host(std::env::var("POSTGRESQL_HOST")?.as_str())
    .connect(postgres::NoTls)?)
}

#[cfg(test)]
pub fn db_in_memory() -> Result<rusqlite::Connection> {
  Ok(rusqlite::Connection::open_in_memory()?)
}

