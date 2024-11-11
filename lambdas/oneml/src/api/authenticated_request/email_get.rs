use super::*;

#[derive(serde::Serialize)]
pub struct Item {
  email: String,
  user_id: String,
  status: constructs::email::Status,
  count_all_time: u32,
}

#[derive(derive_sql::DeriveSqlStatement)]
struct Count {
  email: String,
  count: u32,
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

pub async fn implementation<T, C, R>(store: &T, conn: &mut C, identity: Identity) -> Result<Vec<Item>>
where T: traits::store::EmailStore, C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
{
    use derive_sql::traits::SelectV2;
    log::info!("[GET] ApiEmail: Load emails");
    let emails = constructs::Email::list_from_identity(&identity, store).await?;
    let counts: Vec<Count> = SqlCountHelper { user_id: identity.id.as_str() }.select(conn)?;

    let result = emails.into_iter()
    .map(|e| {
      let count_all_time = counts.iter().find(|c| c.email.eq(e.email.as_str())).map(|c| c.count).unwrap_or(0);
      Item {
        email: e.email,
        user_id: e.user_id,
        status: e.status,
        count_all_time, 
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
        &db::tables::Email::from_id_message_id_subject_from_to_forwarded(
          "u1", "m1", "Message 1", "someone@home.com", "anything@u1.two.com", false,
        ))?;
      sql.insert(conn,
        &db::tables::Email::from_id_message_id_subject_from_to_forwarded(
          "u1", "m2", "Message 2", "someone@home.com", "anything@u1.two.com", false,
        ))?;
      sql.insert(conn,
        &db::tables::Email::from_id_message_id_subject_from_to_forwarded(
          "--", "m3", "Message 3", "someone@home.com", "anything@mm.two.com", false,
        ))?;

      /* Run implementation tests */
      let r = implementation(store, conn,
        Identity::from_id_username_email_emailverified("u1", "u1 name", "one@home.com", true)
      ).await?;
      assert!(r.len() == 1);
      assert!(r[0].email.eq("anything@u1.two.com"));
      assert!(r[0].count_all_time == 2);

      let r = implementation(store, conn,
        Identity::from_id_username_email_emailverified("u2", "u2 name", "two@home.com", true)
      ).await?;
      assert!(r.len() == 0);

      let r = implementation(store, conn,
        Identity::from_id_username_email_emailverified("u3", "u3 name", "three@home.com", true)
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
