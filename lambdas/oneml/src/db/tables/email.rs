use super::*;

/// Email data structure used in database 
#[derive(serde::Serialize, derive_sql::DeriveSqlStatement)]
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
  pub fn from_id_message_id_subject_from_to_forwarded(id: &str, message_id: &str, subject: &str, from: &str, to: &str, forwarded: bool) -> Email {
    Email {
      id: id.to_string(),
      message_id: message_id.to_string(),
      date: chrono::Utc::now().naive_local(),
      subject: subject.to_string(), 
      from: from.to_string(), 
      to: to.to_string(), 
      forwarded,
    }
  }
}

#[cfg(test)]
impl Email {
  pub fn from_id_message_id_date_subject_from_to_forwarded(id: &str, message_id: &str, date: &chrono::naive::NaiveDateTime, subject: &str, from: &str, to: &str, forwarded: bool) -> Email {
    Email {
      id: id.to_string(),
      message_id: message_id.to_string(),
      date: date.clone(),
      subject: subject.to_string(), 
      from: from.to_string(), 
      to: to.to_string(), 
      forwarded,
    }
  }
}
