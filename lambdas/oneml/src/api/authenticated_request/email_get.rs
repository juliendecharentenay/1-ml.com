use super::*;

#[derive(serde::Serialize)]
pub struct Item {
  email: String,
  status: constructs::email::Status,
  count_all_time: u32,
  count_6_days: u32,
  last_email: Option<String>,
}

struct LastEmail {
  email: String,
  date: String,
}
impl<R> derive_sql::traits::TryFromRefRow<R> for LastEmail
where R: derive_sql::traits::Row
{
  fn try_from(row: &R) -> derive_sql::Result<Self> {
    let date: String = row.get(1).ok_or(derive_sql::Error::RowItemNotFound(1))??;
    Ok(LastEmail {
      email: row.get(0).ok_or(derive_sql::Error::RowItemNotFound(0))??,
      date, // : chrono::naive::NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M:%S")?,
    })
  }
}

struct SqlLastEmailHelper<'a> { user_id: &'a str, }
impl<'a> derive_sql::traits::SelectStatement for SqlLastEmailHelper<'a> {
  fn select_stmt(&self) -> derive_sql::Result<String> {
    Ok(format!("
SELECT `{to}`,MAX({date}) FROM {email}
WHERE {id}='{user_id}'
      ",
      email=db::tables::SqlEmail::TABLE_NAME,
      id=db::tables::SqlEmail::ID,
      to=db::tables::SqlEmail::TO,
      date=db::tables::SqlEmail::DATE,
      user_id=self.user_id,
      ))
   }
}


#[derive(derive_sql::DeriveSqlStatement)]
struct Count {
  email: String,
  count: u32,
}

struct SqlCountSinceHelper<'a> { user_id: &'a str, since: &'a chrono::naive::NaiveDateTime, }
impl<'a> derive_sql::traits::SelectStatement for SqlCountSinceHelper<'a> {
  fn select_stmt(&self) -> derive_sql::Result<String> {
    Ok(format!("
SELECT `{to}`,COUNT(*) FROM {email}
WHERE {id}='{user_id}'
  AND {date}>='{since}'
GROUP BY `{to}`
      ",
      email=db::tables::SqlEmail::TABLE_NAME,
      id=db::tables::SqlEmail::ID,
      to=db::tables::SqlEmail::TO,
      date=db::tables::SqlEmail::DATE,
      user_id=self.user_id,
      since=self.since,
    ))
  }
}

struct SqlCountHelper<'a> {
  user_id: &'a str,
}
impl<'a> derive_sql::traits::SelectStatement for SqlCountHelper<'a> {
  fn select_stmt(&self) -> derive_sql::Result<String> {
    Ok(format!("
SELECT `{to}`,COUNT(*) FROM {email}
WHERE {id}='{user_id}'
GROUP BY `{to}`
      ",
      email=db::tables::SqlEmail::TABLE_NAME,
      id=db::tables::SqlEmail::ID,
      to=db::tables::SqlEmail::TO,
      user_id=self.user_id,
    ))
  }
}

pub async fn implementation<T, C, R>(store: &T, conn: &mut C, identity: Identity, today: chrono::naive::NaiveDateTime) -> Result<Vec<Item>>
where T: traits::store::EmailStore, C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
{
    use derive_sql::traits::SelectV2;
    log::info!("[GET] ApiEmail: Load emails");
    let emails = constructs::Email::list_from_identity(&identity, store).await?;
    let counts_all_time: Vec<Count> = SqlCountHelper { user_id: identity.id.as_str() }.select(conn)?;
    let counts_6_days: Vec<Count> 
    = SqlCountSinceHelper { user_id: identity.id.as_str(), 
      since: &today.checked_sub_days(chrono::Days::new(6)).ok_or(Error::DateOutOfRange)?,
    }.select(conn)?;
    let last_emails: Vec<LastEmail> 
    = if counts_all_time.len() > 0 { 
      SqlLastEmailHelper { user_id: identity.id.as_str() }.select(conn)?
    } else {
      Vec::new()
    };

    let result = emails.into_iter()
    .map(|e| {
      let count_all_time = counts_all_time.iter().find(|c| c.email.eq(e.email.as_str())).map(|c| c.count).unwrap_or(0);
      let count_6_days = counts_6_days.iter().find(|c| c.email.eq(e.email.as_str())).map(|c| c.count).unwrap_or(0);
      let last_email = last_emails.iter().find(|c| c.email.eq(e.email.as_str())).map(|c| c.date.clone());
      Item {
        email: e.email,
        status: e.status,
        count_all_time, 
        count_6_days,
        last_email,
      }
    })
    .collect::<Vec<Item>>();

    Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn it_gets_email() -> Result<()> {
    async fn test<T, C, R>(store: &T, conn: &mut C) -> Result<()>
    where T: traits::store::EmailStore, C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
    {
      use derive_sql::traits::{Table, Insert, };

      /* Initialise databases */
      let _ = store.save_email(
        constructs::email::Email::from_email_userid_status("anything@u1.two.com", "u1", constructs::email::Status::Forward)
      ).await?;
      let _ = store.save_email(
        constructs::email::Email::from_email_userid_status("anything@u3.two.com", "u3", constructs::email::Status::Forward)
      ).await?;
      let sql = db::tables::SqlEmail::default();
      sql.create(conn)?;
      sql.insert(conn,
        &db::tables::Email::from_id_message_id_date_subject_from_to_forwarded(
          "u1", "m1", 
          &chrono::naive::NaiveDate::from_ymd_opt(2024, 11, 12).unwrap()
          .and_time(chrono::naive::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
          "Message 1", "someone@home.com", "anything@u1.two.com", false,
        ))?;
      sql.insert(conn,
        &db::tables::Email::from_id_message_id_date_subject_from_to_forwarded(
          "u1", "m2", 
          &chrono::naive::NaiveDate::from_ymd_opt(2024, 10, 11).unwrap()
          .and_time(chrono::naive::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
          "Message 2", "someone@home.com", "anything@u1.two.com", false,
        ))?;
      sql.insert(conn,
        &db::tables::Email::from_id_message_id_date_subject_from_to_forwarded(
          "--", "m3", 
          &chrono::naive::NaiveDate::from_ymd_opt(2024, 11, 10).unwrap()
          .and_time(chrono::naive::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
          "Message 3", "someone@home.com", "anything@mm.two.com", false,
        ))?;

      /* Run implementation tests */
      let r = implementation(store, conn,
        Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true),
        chrono::naive::NaiveDate::from_ymd_opt(2024, 11, 13).unwrap()
        .and_time(chrono::naive::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
      ).await?;
      assert!(r.len() == 1);
      assert!(r[0].email.eq("anything@u1.two.com"));
      assert!(r[0].count_all_time == 2);
      assert!(r[0].count_6_days == 1);
      // assert!(r[0].last_email.unwrap().date() == chrono::naive::NaiveDate::from_ymd_opt(2024, 11, 12).unwrap());

      let r = implementation(store, conn,
        Identity::from_id_username_email_emailverified("u2", "u2 name", "two@home.com", true),
        chrono::naive::NaiveDate::from_ymd_opt(2024, 11, 13).unwrap()
        .and_time(chrono::naive::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
      ).await?;
      assert!(r.len() == 0);

      let r = implementation(store, conn,
        Identity::from_id_username_email_emailverified("u3", "u3 name", "three@home.com", true),
        chrono::naive::NaiveDate::from_ymd_opt(2024, 11, 13).unwrap()
        .and_time(chrono::naive::NaiveTime::from_hms_opt(11, 0, 0).unwrap()),
      ).await?;
      assert!(r.len() == 1);
      assert!(r[0].email.eq("anything@u3.two.com"));
      assert!(r[0].count_all_time == 0);

      Ok(())
    }

    let store = traits::store::email::mock::EmailStoreMockBuilder::default().build().unwrap();
    let mut conn = db::db_in_memory()?;
    test(&store, &mut conn).await?;

    Ok(())
  }
}
