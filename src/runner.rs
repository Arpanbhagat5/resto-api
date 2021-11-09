use super::tables;
use super::status;
use super::menu;
use super::orders;

use diesel::prelude::*;
use diesel::pg::PgConnection;


pub fn create_order(conn: &PgConnection) {
  let table_id = 10;
    let new_order = orders::NewOrder{
        table_id: table_id
    };

    if orders::Orders::create_order(new_order, table_id, &conn) {
        println!("Success! order created!");
    }
    else {
        println!("Failed: Table is already occupied!");
    }
}

pub fn add_item_to_order(conn: &PgConnection) {
  let order_id = 9;
  let item_id = 10; // 10-15
  let prep_time = 10; // 5,10,15
  let status_id = 5; // 5-8

  // TODO: check for validity of ids
  let new_order_item = orders::NewOrderItem{
    order_id: order_id,
    item_id: item_id,
    prep_time: prep_time,
    status_id: status_id
  };
  let result = orders::Orders::add_item_to_order(new_order_item.order_id, new_order_item, &conn);
  if result {
    println!{"Order created successfully!"};
    // TODO: Call sleeper thread to simulate "ordered->preparing" flow
    // Handle "canceled" interrupt if thread still running
    // Mark order_item::status as "served" at end of sleep period
  }
  else {
    println!{"Order creation failed"};
  }
}

pub fn set_table_free(conn: &PgConnection) {
  tables::Tables::set_table_free(10, &conn);
}

// pub fn get_full_order_list_for_table(conn: &PgConnection) {
//   let result: Result<Vec<OrderItems>, diesel::result::Error> = orders::Orders::get_full_order_list_for_table(10, &conn);
// }

