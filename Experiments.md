# update with struct without id
```
use schema::menu::dsl::{item as i, prep_time as pt}; //can use this short notation
let NewItem {
    item,
    prep_time
} = target_item;

diesel::update(menu_items.find(id))
    .set((item.eq(item), prep_time.eq(prep_time)))
    .get_result::<Menu>(conn)
    .is_ok()
```

# Implement getter with std input as a param value¥
```
  let mut result: Vec<models::Menu> = Vec::new();
  println!("Please input the id you want to search for.");
  let mut search_id = String::new();

  io::stdin()
      .read_line(&mut search_id)
      .expect("Failed to get id");
  result = models::Menu::get_item(search_id.parse::<i32>().unwrap(), &conn);
```