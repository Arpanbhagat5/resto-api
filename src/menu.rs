use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::menu;
use super::schema::menu::dsl::menu as menu_items;

use super::db_calls;


// Adding Queryable to make this struct to query DB
#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "menu"]
pub struct Menu {
  pub item_id: i32,
  pub item : String,
  pub calories : i32,
}

// Will be used to create a new instance of item for insert/update op
#[derive(Queryable, Insertable, Debug)]
#[table_name = "menu"]
pub struct NewMenuItem {
  pub item : String,
  pub calories : i32,
}

impl Menu {
  // Get a single item
  pub fn get_item(item_id: i32, conn: &PgConnection) -> Vec<Menu> {
    return db_calls::get_item_from_menu(item_id, &conn)
  }

  // Get all items
  pub fn get_all_items(conn: &PgConnection) -> Vec<Menu> {
    return db_calls::get_all_items_from_menu(&conn)
  }

  // Update a single item by item_id
  // pub fn update_item(item_id: i32, target_item: Menu, conn: &PgConnection) -> bool {
  //   return db_calls::update_item_in_menu(target_item, &conn)
  // }

  pub fn add_item(new_item: NewMenuItem, conn: &PgConnection) -> bool {
    return db_calls::add_item_to_menu(new_item, &conn)
  }

  pub fn delete_item(item_id: i32, conn: &PgConnection) -> bool {
    if Menu::get_item(item_id, conn).is_empty() {
      return false;
    };
    return db_calls::delete_item_from_menu(item_id, &conn)

  }
}