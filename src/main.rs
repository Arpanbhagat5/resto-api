#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;


mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let mut new_item = models:: NewItem {
        item: String::from("Spaghetti"),
        calories: 600,
    };

    // Insert an object: Working well
    if models::Menu::add_item(new_item, &conn) {
        println!("Successfully added");
    }
    else {
        println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    }

    let mut new_item = models:: NewItem {
        item: String::from("Pizza"),
        calories: 900,
    };

    // Insert an object: Working well
    if models::Menu::add_item(new_item, &conn) {
        println!("Successfully added");
    }
    else {
        println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    }

    let mut new_item = models:: NewItem {
        item: String::from("Pasta"),
        calories: 500,
    };

    // Insert an object: Working well
    if models::Menu::add_item(new_item, &conn) {
        println!("Successfully added");
    }
    else {
        println!("Failed! Possible reason: Incorrect data type passed or connection failure");
    }

    // Get one item with 'id'
    let mut result: Vec<models::Menu> = Vec::new();
    println!("Single item get test");
    result = models::Menu::get_item(2, &conn);
    println!("{:?}", result);

    // Get all items : Working well
    println!("All items get test");
    // let mut result: Vec<models::Menu> = Vec::new();
    result = models::Menu::get_all_items(&conn);
    println!("All items of menu are: {:?}", result);

    // Delete an item
    println!("Delete item test");
    if models::Menu::delete_item(4, &conn) {
        println!("Successfully deleted");
    }
    else {
        println!("Failed! possible reasons: id doesnt exist");
    }

    // Update an item
    let mut target_item = models::Menu {
        id: 1,
        item: String::from("Ramen"),
        calories: 800
    };
    if models::Menu::update_item_by_id(1, target_item, &conn) {
        println!("Successfully Updated");
    }
    else {
        println!("Failed! possible reasons: id doesnt exist");
    }
}
