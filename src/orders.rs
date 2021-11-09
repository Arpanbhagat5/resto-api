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

use crate::tables::Tables;
use crate::orders::orders::dsl::orders as all_orders;


// This was a saver at 2AM
#[derive(QueryableByName, Debug)]
struct IntOrder {
    #[sql_type = "Integer"]
    order_id: i32
}

// Used when SQL query needs a single value
#[derive(QueryableByName, Debug)]
struct IntTable {
    #[sql_type = "Integer"]
    table_id: i32
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

#[derive(QueryableByName, Debug)]
struct IntCount {
    #[sql_type = "BigInt"]
    count: i64
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
    let is_occupied = Tables::is_occupied(new_order.table_id, &conn);

    // TODO: handle the case where 2nd query fails, maybe need retry mechanism
    if !is_occupied {
      let order_created = diesel::insert_into(orders::table)
      .values(new_order)
      .execute(conn)
      .is_ok();

      let table_occupied = Tables::set_occupied(table_id, &conn);
      return order_created && table_occupied
    }
    else {
      println!("Table is already occupied!");
      return false
    }
  }

  pub fn add_item_to_order(order_id: i32, new_order_item: NewOrderItem, conn: &PgConnection) -> bool {
    println!("{:?}", new_order_item); //How come the obj is still accessible in diesel query below?
    diesel::insert_into(order_items::table)
      .values(new_order_item)
      .execute(conn)
      .is_ok()
  }

  pub fn get_order_id_from_table_id(table_id: i32, conn: &PgConnection) -> i32 {
    let result:Vec<IntOrder> = sql_query("SELECT order_id FROM orders WHERE table_id=$1")
      .bind::<Integer, _>(table_id)
      .load(conn)
      .unwrap();
    let order_id: i32 = result[0].order_id;
    return order_id
  }

  pub fn are_all_orders_served(table_id: i32, conn: &PgConnection) -> bool {
    // TODO: Handle incorrect table_id
    // Get order_id from table_id
    let order_id = Orders::get_order_id_from_table_id(table_id, &conn);

    // Get count of orders not "served"
    let result:Vec<IntCount> = sql_query("SELECT COUNT(order_id) FROM order_items WHERE order_id=$1 AND status_id=5") //TODO status!=8
      .bind::<Integer, _>(order_id)
      .load(conn)
      .unwrap();
    let count = result[0].count;

    // if count of not "served" return false
    if count > 0 {
      return false
    }
    else {
      return true
    }
  }

  // pub fn get_full_order_list_for_table(table_id: i32, conn: &PgConnection) -> Vec<OrderItems> {
  //   let order_num = Orders::get_order_id_from_table_id(table_id, &conn);

  //   let result: Vec<OrderItems> = sql_query("SELECT * FROM order_items WHERE order_id=$1")
  //     .bind::<Integer, _>(order_id)
  //     .get_results(conn)
  //     .unwrap();
  //   println!("{:?}", result);

    // use self::order_items::dsl::*;
    // let result: Vec<OrderItems> = order_items
    //   .filter(order_id.eq(&order_num))
    //   .load(conn);
    // return result
  // }

  // pub fn get_order_list_for_table_by_status(table_id: i32, status_id: i32, conn: &PgConnection) -> Vec<OrderItems> {
  //   let order_id = Orders::get_order_id_from_table_id(table_id, &conn);
  //   let result:Vec<OrderItems> = sql_query("SELECT * FROM order_items WHERE order_id=$1 AND status_id=$2") // status!=8
  //     .bind::<Integer, _>(order_id)
  //     .bind::<Integer, _>(status_id)
  //     .load(conn)
  //     .unwrap();
  // }

    // fn get_order_items_row_id_from_item_name(order_id: i32, item_id: i32, conn: &PgConnection) -> Vec<OrderItems> {
  //   order_items::table
  //     .filter((order_id.eq(&order_id), item_id.eq(&item_id)))
  //     .load::<OrderItems>(conn)
  //     .expect("No entry for such item name for specified order_id")
  // }

  // pub fn delete_item_from_order(order_id: i32, item: String, conn: &PgConnection) -> bool {

  //   let order_row = Orders::get_order_items_row_id_from_item_name(order_id, 1);
  //   diesel::delete(order_items::table.find(id.eq(&order_row.id)))
  //     .execute(conn)
  //     .is_ok()
  // }

}