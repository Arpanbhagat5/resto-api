use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;

use diesel::types::ToSql;
use diesel::sql_types::Bool;

use crate::orders::Orders;

use super::schema::tables;
use super::schema::tables::dsl::tables as tables_items;

use super::db_calls;


#[derive(Queryable, Debug)]
pub struct Tables {
  pub table_id: i32,
  pub table_name: String,
  pub is_table_occupied: bool,
}

#[derive(Queryable, Debug, Insertable)]
#[table_name = "tables"]
pub struct NewTable {
  pub table_name: String,
  pub is_table_occupied: bool,
}


impl Tables {
  pub fn add_table(new_table: NewTable, conn: &PgConnection ) -> bool {
    db_calls::add_table(new_table, &conn)
  }

  pub fn is_table_occupied(table_id: i32, conn: &PgConnection ) -> bool {
    db_calls::is_table_occupied(table_id, &conn)
  }

  pub fn set_table_occupied(table_id: i32, conn: &PgConnection ) -> bool {
    db_calls::set_table_occupied(table_id, &conn)
  }

  pub fn set_table_free(table_id: i32, conn: &PgConnection ) {
    // Check if all orders are "served"
    if Orders::are_all_orders_served(table_id, &conn) {
      // Set table free by setting is_table_occupied flag as false
      if db_calls::set_table_free(table_id, &conn) {
        println!("Table is free now");
      }
    }
    else {
      println!("Cannot set free: Table has unserved items");
    }
  }
}