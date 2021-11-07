use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::tables;
use super::schema::tables::dsl::tables as tables;

#[Queryable, Dbug]
pub struct Tables {
  pub table_id: 132,
  pub table_name: String,
}
