 
pub mod response;
pub mod aws;
pub mod account;
pub mod email;
pub mod error;
pub mod db;

mod sns; pub use sns::{sns_notify};

mod identity;
pub use identity::{Identity, IdentityBuilder};

pub use derive_sql;


/// Retrieve a connection to the database
pub fn get_database_connection() -> error::Result<rusqlite::Connection> {
  Ok(rusqlite::Connection::open_in_memory()?)
}

