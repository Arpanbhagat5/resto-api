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
mod runner;


fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    // init_db::init_tables(&conn);
    // init_db::init_status(&conn);
    // init_db::init_menu(&conn);

    // runner::create_order(&conn);
    // runner::add_item_to_order(&conn);
    runner::set_table_free(&conn);
    // runner::get_full_order_list_for_table(&conn); not working


}
