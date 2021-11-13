use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::status;

use super::db_calls;

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
    db_calls::add_status_to_db(new_status, &conn)
  }

  pub fn get_status_id_from_status(status: String, conn: &PgConnection) -> i32 {
    let result = db_calls::get_status_id_from_status(status, &conn);
    return result[0].status_id
  }

}