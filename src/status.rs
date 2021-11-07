use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::status;

#[derive(Queryable, Debug)]
pub struct Status {
  pub status_id: i32,
  pub description: String,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "status"]
pub struct NewStatus {
  pub description: String,
}

impl Status {
  pub fn add_status(new_status: NewStatus, conn: &PgConnection) -> bool {
    diesel::insert_into(status::table)
      .values(new_status)
      .execute(conn)
      .is_ok()
  }
}