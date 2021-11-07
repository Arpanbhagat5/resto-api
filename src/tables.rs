use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::tables;


#[derive(Queryable, Debug)]
pub struct Tables {
  pub table_id: i32,
  pub table_name: String,
}


#[derive(Queryable, Debug, Insertable)]
#[table_name = "tables"]
pub struct NewTable {
  pub table_name: String,
}

impl Tables {
  pub fn add_table(new_table: NewTable, conn: &PgConnection ) -> bool {
    diesel::insert_into(tables::table)
      .values(new_table)
      .execute(conn)
      .is_ok()
  }
}