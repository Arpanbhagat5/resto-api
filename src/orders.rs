#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Tables)]
pub struct Orders {
  pub order_id: 132,
  pub table_id: i32,
}
