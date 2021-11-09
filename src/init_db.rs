use super::tables;
use super::status;
use super::menu;
use diesel::pg::PgConnection;


// for table intitialization
pub fn init_tables(conn: &PgConnection) {
  for num in 1..101 {
    let mut table_name = String::from("Table_");
    let table_num_suffix = num.to_string();
    table_name.push_str(&table_num_suffix);

    let new_table = tables::NewTable {
      table_name: table_name,
      is_occupied: false
    };

    if tables::Tables::add_table(new_table, &conn) {
      println!("Successfully added");
    }
    else {
      println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    }
  }
}

pub fn init_status(conn: &PgConnection) {
  let status_list: Vec<&str> = vec!["ordered", "preparing", "served", "cancelled"];
  for status in status_list.iter() {
    let new_status = status::NewStatus {
      description: status.to_string()
    };

    if status::Status::add_status(new_status, &conn) {
      println!("Successfully added");
    }
    else {
      println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    }
  }
}

pub fn init_menu(conn: &PgConnection) {
  let item_list: Vec<(&str, i32)> = vec![("Pizza", 900), ("Pasta", 800), ("Spaghetti", 900), ("Samosa", 300), ("Juice", 250), ("Ice-cream", 600)];
  for tuple in item_list.iter() {
    let new_item = menu::NewItem {
      item: tuple.0.to_string(),
      calories: tuple.1
    };

    if menu::Menu::add_item(new_item, &conn) {
      println!("Successfully added");
    }
    else {
      println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    }
  }
}