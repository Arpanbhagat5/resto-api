use diesel;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::sql_query;
use diesel::sql_types::*;

use super::schema::tables;
use super::schema::status;
use super::schema::menu;
use super::schema::orders;
use super::schema::order_items;

use super::db_calls;

use crate::tables::Tables;
// use crate::orders::orders::dsl::orders as all_orders;

use crate::status::Status;

// Used when SQL query needs a single value
#[derive(QueryableByName, Debug)]
struct IntOrder {
    #[sql_type = "Integer"]
    order_id: i32
}

#[derive(QueryableByName, Debug)]
struct IntTable {
  #[sql_type = "Integer"]
  table_id: i32
}

#[derive(QueryableByName, Debug)]
struct IntCount {
    #[sql_type = "BigInt"]
    count: i64
}


#[derive(Identifiable, Queryable, Insertable)]
#[primary_key(order_id)]
#[table_name = "orders"]
pub struct Orders {
  pub order_id: i32,
  pub table_id: i32,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "orders"]
pub struct NewOrder {
  pub table_id: i32,
}

#[derive(Identifiable, Queryable, QueryableByName, Debug)]
#[primary_key(id)]
#[table_name = "order_items"]
pub struct OrderItems {
  pub id: i32,
  pub prep_time: i32,
  pub order_id: i32,
  pub item_id: i32,
  pub status_id: i32,
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "order_items"]
pub struct NewOrderItem {
  pub order_id: i32,
  pub item_id: i32,
  pub prep_time: i32,
  pub status_id: i32,
}

impl Orders {
  pub fn create_order(new_order: NewOrder, table_id: i32, conn: &PgConnection) -> bool {
    let is_table_occupied = Tables::is_table_occupied(new_order.table_id, &conn);

    // TODO: handle the case where 2nd query fails, maybe need retry mechanism
    if !is_table_occupied {
      let order_created = db_calls::create_order_for_table(new_order, table_id, &conn);
      let table_occupied = Tables::set_table_occupied(table_id, &conn);
      return order_created && table_occupied
    }
    else {
      println!("Table is already occupied!");
      return false
    }
  }

  pub fn add_item_to_order(order_id: i32, new_order_item: NewOrderItem, conn: &PgConnection) -> bool {
    db_calls::add_item_to_order(order_id, new_order_item, &conn)
  }

  pub fn get_order_id_from_table_id(table_id: i32, conn: &PgConnection) -> i32 {
    let result:Vec<db_calls::IntOrder> = db_calls::get_order_id_from_table_id(table_id, &conn);
    let order_id: i32 = result[0].order_id;
    return order_id
  }

  pub fn are_all_orders_served(table_id: i32, conn: &PgConnection) -> bool {
    // TODO: Handle incorrect table_id
    let order_id = Orders::get_order_id_from_table_id(table_id, &conn);

    // Get count of orders not "served"
    let served_status_id = Status::get_status_id_from_status(String::from("served"), &conn);
    let result:Vec<db_calls::IntCount> = db_calls::get_order_item_count_for_not_status(order_id, served_status_id, &conn);
    let unserved_items = result[0].count;

    // if count of not "served" return false
    if unserved_items > 0 {
      return false
    }
    else {
      return true
    }
  }

  pub fn get_order_list_for_table(table_id: i32, conn: &PgConnection) {
    let order_num = Orders::get_order_id_from_table_id(table_id, &conn);

    let result: Vec<OrderItems> = db_calls::get_order_list_for_table(order_num, &conn);
    println!("{:?}", result);
  }

}