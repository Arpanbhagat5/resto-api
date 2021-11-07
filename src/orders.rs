// use diesel;
// use diesel::prelude::*;
// use diesel::pg::PgConnection;


// use super::schema::tables;
// use super::schema::status;

// // use super::schema::orders;
// use super::schema::orders::dsl::orders as orders;

// // use super::schema::order_items;
// use super::schema::order_items::dsl::order_items as order_items;


// #[derive(Insertable, Identifiable, Queryable, Associations)]
// #[belongs_to(Table, foreign_key="table_id")]
// #[primary_key(order_id)]
// #[table_name = "orders"]
// pub struct Orders {
//   pub order_id: i32,
//   pub table_id: i32,
// }


// #[derive(Queryable, Insertable, Debug)]
// #[table_name = "orders"]
// pub struct NewOrder {
//   pub table_id: i32,
// }


// #[derive(Identifiable, Queryable, Associations)]
// #[belongs_to(Orders, foreign_key="order_id")]
// #[belongs_to(Menu, foreign_key="item_id")]
// #[belongs_to(Status, foreign_key="status_id")]
// #[primary_key(id)]
// pub struct OrderItems {
//   pub id: i32,
//   pub prep_time: i32,
//   pub order_id: i32,
//   pub item_id: i32,
//   pub status_id: i32,

// }

// pub struct NewOrderItemsRow {
//   pub order_id: i32,
//   pub item_id: i32,
//   pub prep_time: i32,
//   pub status_id: i32,
// }

// impl Orders{
//   pub fn create_order(new_order: NewOrder, conn: &PgConnection) -> bool {
//     diesel::insert_into(orders)
//       .values(new_order)
//       .execute(conn)
//       .is_ok()
//   }

//   pub fn add_item_to_order(order_id: i32, new_row: NewOrderItemsRow, conn: &PgConnection) -> bool {
//     diesel::insert_into(order_items)
//       .values(new_row)
//       .execute(conn)
//       .is_ok()
//   }

//   fn get_order_items_row_id_from_item_name(order_id: i32, item_name: String) -> Vec<OrderItems> {
//     order_items
//       .filter(order_items::order_id.eq(order_id), order_items::item_name.eq(item_name))
//       .load::<OrderItems>(&conn)
//       .expect("No entry for such item name for specified order_id")
//   }

//   pub fn delete_item_from_order(order_id: i32, item_name: String, conn: &PgConnection) -> bool {
//     order_row = self.get_order_items_row_id_from_item_name(order_id, item_name);
//     diesel::delete(order_items.find(order_row.id))
//       .execute(&conn)
//       .is_ok()
//   }

//   // pub fn get_order_items_by_table(table_id: i32, connection: &PgConnection) -> Vec(OrderItems) {
//   // }
// }