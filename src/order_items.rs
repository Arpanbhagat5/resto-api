#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Orders, foreign_key="order_id")]
#[belongs_to(Menu, foreign_key="item_id")]
#[belongs_to(Status, foreign_key="status_id")]
pub struct OrderItems {
  pub id: 132,
  pub prep_time i32,
  pub order_id: i32,
  pub item_id: i32,
  pub status_id: i32,

}