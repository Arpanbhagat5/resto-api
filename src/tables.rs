use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;

use diesel::types::ToSql;
use diesel::sql_types::Bool;

use crate::orders::Orders;

use super::schema::tables;
use super::schema::tables::dsl::tables as tables_items;


#[derive(Queryable, Debug)]
pub struct Tables {
  pub table_id: i32,
  pub table_name: String,
  pub is_occupied: bool,
}


#[derive(Queryable, Debug, Insertable)]
#[table_name = "tables"]
pub struct NewTable {
  pub table_name: String,
  pub is_occupied: bool,
}

impl Tables {
  pub fn add_table(new_table: NewTable, conn: &PgConnection ) -> bool {
    diesel::insert_into(tables::table)
      .values(new_table)
      .execute(conn)
      .is_ok()
  }

  pub fn is_occupied(table_id: i32, conn: &PgConnection ) -> bool {
    tables_items
      .find(table_id)
      .select(self::tables::is_occupied)
      .get_result(conn)
      .expect("Could not get table with specified id!")
  }

  pub fn set_occupied(table_id: i32, conn: &PgConnection ) -> bool {
    diesel::update(tables_items.find(table_id))
      .set(self::tables::is_occupied.eq(true))
      .get_result::<Tables>(conn)
      .is_ok()
  }

  pub fn set_table_free(table_id: i32, conn: &PgConnection ) {
    // Check if all orders are "served"
    let all_orders_served = Orders::are_all_orders_served(10, &conn);

    // Set table free by setting is_occupied flag as false
    if all_orders_served {
      diesel::update(tables_items.find(table_id))
        .set(self::tables::is_occupied.eq(false))
        .get_result::<Tables>(conn)
        .is_ok();
      println!("Table is free now");
    }
    else {
      println!("Cannot set free: Table has unserved items");
    }
  }
}