use super::*;

/// Email data structure used in database 
#[derive(derive_sql::DeriveSqlStatement)]
pub struct Email {
  id: String,
  message_id: String,
  date: chrono::naive::NaiveDateTime,
  subject: String,
  from: String,
  to: String,
  forwarded: bool,
}

impl Email {
  pub fn from_id_message_id_subject_from_to_forwarded(id: String, message_id: String, subject: String, from: String, to: String, forwarded: bool) -> Email {
    Email {
      id,
      message_id,
      date: chrono::Utc::now().naive_local(),
      subject, from, to, forwarded,
    }
  }
}
