#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;


mod schema;
mod menu;
mod orders;
mod tables;
mod status;
mod init_db;


fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();


    init_db::init_tables(&conn);
    init_db::init_status(&conn);
    init_db::init_menu(&conn);


    // let mut new_item = menu:: NewItem {
    //     item: String::from("Spaghetti"),
    //     calories: 600,
    // };

    // // Insert an object: Working well
    // if menu::Menu::add_item(new_item, &conn) {
    //     println!("Successfully added");
    // }
    // else {
    //     println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    // }

    // let mut new_item = menu:: NewItem {
    //     item: String::from("Pizza"),
    //     calories: 900,
    // };

    // // Insert an object: Working well
    // if menu::Menu::add_item(new_item, &conn) {
    //     println!("Successfully added");
    // }
    // else {
    //     println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    // }

    // let mut new_item = menu:: NewItem {
    //     item: String::from("Pasta"),
    //     calories: 500,
    // };

    // // Insert an object: Working well
    // if menu::Menu::add_item(new_item, &conn) {
    //     println!("Successfully added");
    // }
    // else {
    //     println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    // }

    // // Get one item with 'item_id'
    // let mut result: Vec<menu::Menu> = Vec::new();
    // println!("Single item get test");
    // result = menu::Menu::get_item(2, &conn);
    // println!("{:?}", result);

    // // Get all items : Working well
    // println!("All items get test");
    // // let mut result: Vec<menu::Menu> = Vec::new();
    // result = menu::Menu::get_all_items(&conn);
    // println!("All items of menu are: {:?}", result);

    // // Delete an item
    // println!("Delete item test");
    // if menu::Menu::delete_item(4, &conn) {
    //     println!("Successfully deleted");
    // }
    // else {
    //     println!("Failed! possible reasons: item_id doesnt exist");
    // }

    // // Update an item
    // let mut target_item = menu::Menu {
    //     item_id: 1,
    //     item: String::from("Ramen"),
    //     calories: 800
    // };
    // if menu::Menu::update_item(1, target_item, &conn) {
    //     println!("Successfully Updated");
    // }
    // else {
    //     println!("Failed! possible reasons: item_id doesnt exist");
    // }


}
