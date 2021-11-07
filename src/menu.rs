use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::menu;
use super::schema::menu::dsl::menu as menu_items;

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
pub struct NewItem {
  pub item : String,
  pub calories : i32,
}

impl Menu {
  // Get a single item
  pub fn get_item(item_id: i32, conn: &PgConnection) -> Vec<Menu> {
    menu_items
      .find(item_id) // Find item with this item_id
      .load::<Menu>(conn) // Cast as Menu type
      .expect("Could not find this item!") // Error out with this line in case of failure
  }

  // Get all items
  pub fn get_all_items(conn: &PgConnection) -> Vec<Menu> {
    menu_items
      .order(menu::item_id.desc())
      .load::<Menu>(conn)
      .expect("Could not get all items!")
  }

  // Update a single item by item_id
  pub fn update_item(item_id: i32, target_item: Menu, conn: &PgConnection) -> bool {

    diesel::update(menu_items.find(item_id))
      .set(&target_item)
      .get_result::<Menu>(conn)
      .is_ok()
  }

  pub fn add_item(new_item: NewItem, conn: &PgConnection) -> bool {
    diesel::insert_into(menu_items)
      .values(&new_item)
      .execute(conn)
      .is_ok()
  }

  pub fn delete_item(item_id: i32, conn: &PgConnection) -> bool {
    if Menu::get_item(item_id, conn).is_empty() {
      return false;
    };
    diesel::delete(menu_items.find(item_id))
        .execute(conn)
        .is_ok()
  }
}