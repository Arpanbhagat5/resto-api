use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::sql_query;
use diesel::sql_types::*;

// for calling all structs
use crate::menu::*;

use super::schema::menu as menu;
use super::schema::menu::dsl::menu as menu_items;

use crate::tables::*;
use super::schema::tables;
use super::schema::tables::dsl::tables as tables_items;

use crate::orders::*;
use super::schema::orders;
use super::schema::order_items;

use crate::status::*;
use super::schema::status;

// This was a saver at 2AM
#[derive(QueryableByName, Debug)]
pub struct IntOrder {
    #[sql_type = "Integer"]
    pub order_id: i32
}

// Used when SQL query needs a single value
#[derive(QueryableByName, Debug)]
pub struct IntTable {
    #[sql_type = "Integer"]
    pub table_id: i32
}

#[derive(QueryableByName, Debug)]
pub struct IntCount {
    #[sql_type = "BigInt"]
    pub count: i64
}

#[derive(QueryableByName, Debug)]
pub struct IntStatus {
    #[sql_type = "Integer"]
    pub status_id: i32
}

/*
Status related queries
*/
pub fn add_status_to_db(new_status: NewStatus, conn: &PgConnection) -> bool {
  diesel::insert_into(status::table)
    .values(new_status)
    .execute(conn)
    .is_ok()
}

pub fn get_status_id_from_status(status: String, conn: &PgConnection) -> Vec<IntStatus> {
  sql_query("SELECT id FROM status WHERE description=$1")
    .bind::<Text, _>(status)
    .load(conn)
    .unwrap()
}

/*
Menu related queries
*/
pub fn get_item_from_menu(item_id: i32, conn: &PgConnection) -> Vec<Menu> {
  menu_items
    .find(item_id) // Find item with this item_id
    .load::<Menu>(conn) // Cast as Menu type
    .expect("Could not find this item!") // Error out with this line in case of failure
}

pub fn add_item_to_menu(new_item: NewMenuItem, conn: &PgConnection) -> bool {
  diesel::insert_into(menu_items)
  .values(&new_item)
  .execute(conn)
  .is_ok()
}

// pub fn update_item_in_menu(item_id: i32, target_item: Menu, conn: &PgConnection) -> bool {
//   diesel::update(menu_items.find(item_id))
//   .set(&target_item)
//   .get_result::<Menu>(conn)
//   .is_ok()
// }

pub fn get_all_items_from_menu(conn: &PgConnection) -> Vec<Menu> {
  menu_items
    .order(menu::item_id.desc())
    .load::<Menu>(conn)
    .expect("Could not get all items!")
}

pub fn delete_item_from_menu(item_id: i32, conn: &PgConnection) -> bool {
  diesel::delete(menu_items.find(item_id))
      .execute(conn)
      .is_ok()
}

/*
Table related queries
*/
pub fn add_table(new_table: NewTable, conn: &PgConnection ) -> bool {
  diesel::insert_into(tables::table)
    .values(new_table)
    .execute(conn)
    .is_ok()
}

pub fn is_table_occupied(table_id: i32, conn: &PgConnection ) -> bool {
  tables_items
    .find(table_id)
    .select(self::tables::is_table_occupied)
    .get_result(conn)
    .expect("Could not get table with specified id!")
}

pub fn set_table_occupied(table_id: i32, conn: &PgConnection ) -> bool {
  diesel::update(tables_items.find(table_id))
    .set(self::tables::is_table_occupied.eq(true))
    .get_result::<Tables>(conn)
    .is_ok()
}

pub fn set_table_free(table_id: i32, conn: &PgConnection) -> bool {
  diesel::update(tables_items.find(table_id))
    .set(self::tables::is_table_occupied.eq(false))
    .get_result::<Tables>(conn)
    .is_ok()
}

/*
Order related queries
*/
pub fn create_order_for_table(new_order: NewOrder, table_id: i32, conn: &PgConnection) -> bool {
  diesel::insert_into(orders::table)
  .values(new_order)
  .execute(conn)
  .is_ok()
}

pub fn add_item_to_order(order_id: i32, new_order_item: NewOrderItem, conn: &PgConnection) -> bool {
  diesel::insert_into(order_items::table)
    .values(new_order_item)
    .execute(conn)
    .is_ok()
}

pub fn get_order_id_from_table_id(table_id: i32, conn: &PgConnection) -> Vec<IntOrder> {
  sql_query("SELECT order_id FROM orders WHERE table_id=$1")
    .bind::<Integer, _>(table_id)
    .load(conn)
    .unwrap()
}

pub fn get_order_item_count_for_not_status(order_id: i32, status_id: i32, conn: &PgConnection) -> Vec<IntCount> {
  sql_query("SELECT COUNT(order_id) FROM order_items WHERE order_id=$1 AND status_id!=$2")
    .bind::<Integer, _>(order_id)
    .bind::<Integer, _>(status_id)
    .load(conn)
    .unwrap()
}

pub fn get_order_item_count_for_status(order_id: i32, status_id: i32, conn: &PgConnection) -> Vec<IntCount> {
  sql_query("SELECT COUNT(order_id) FROM order_items WHERE order_id=$1 AND status_id=$2")
    .bind::<Integer, _>(order_id)
    .bind::<Integer, _>(status_id)
    .load(conn)
    .unwrap()
}

pub fn get_order_list_for_table(order_num: i32, conn: &PgConnection) -> Vec<OrderItems> {
  sql_query("SELECT * FROM order_items WHERE order_id=$1")
    .bind::<Integer, _>(order_num)
    .get_results(conn)
    .unwrap()

  // use self::order_items::dsl::*;
  // let result: Vec<OrderItems> = order_items
  //   .filter(order_id.eq(&order_num))
  //   .load(conn);
  // return result
}

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
