use super::*;

pub async fn implementation<C, R>(conn: &mut C, identity: Identity, email: &str) -> Result<Vec<db::tables::Email>>
where C: derive_sql::traits::Connection<R>, R: derive_sql::traits::Row,
{
  use derive_sql::traits::SelectV2;

  log::info!("[GET] ReceveidEmail");
  let r = db::tables::SqlEmail::default()
  .select_with_filter(conn,
    &derive_sql::structs::filter::And::from(
    (
      derive_sql::structs::Field::from(db::tables::SqlEmail::ID).eq(identity.id),
      derive_sql::structs::Field::from(db::tables::SqlEmail::FROM).eq(email),
    )))?;
  Ok(r)
}
