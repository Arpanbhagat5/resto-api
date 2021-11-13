pub fn add_two(a: i32, b: i32) -> i32 {
  a+b
}


#[cfg(test)]
mod tests {
  use crate::orders::*;

  #[test]
  fn it_works() {
      assert_eq!(4, add_two(2,2));
  }

  #[test]
  fn test_create_order() {
    let table_id = 10;
      let new_order = orders::NewOrder{
          table_id: table_id
      };

      assert_eq!(true, orders::Orders::create_order(new_order, table_id, &conn));
  }
}