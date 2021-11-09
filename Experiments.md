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


## Menu ops
```
    // Get one item with 'item_id'
    let mut result: Vec<menu::Menu> = Vec::new();
    println!("Single item get test");
    result = menu::Menu::get_item(2, &conn);
    println!("{:?}", result);

    // Get all items : Working well
    println!("All items get test");
    // let mut result: Vec<menu::Menu> = Vec::new();
    result = menu::Menu::get_all_items(&conn);
    println!("All items of menu are: {:?}", result);

    // Delete an item
    println!("Delete item test");
    if menu::Menu::delete_item(4, &conn) {
        println!("Successfully deleted");
    }
    else {
        println!("Failed! possible reasons: item_id doesnt exist");
    }

    // Update an item
    let mut target_item = menu::Menu {
        item_id: 1,
        item: String::from("Ramen"),
        calories: 800
    };
    if menu::Menu::update_item(1, target_item, &conn) {
        println!("Successfully Updated");
    }
    else {
        println!("Failed! possible reasons: item_id doesnt exist");
    }
```

## return single col after insert
```
use schema::users::dsl::*;

insert_into(users)
    .values(name.eq("Ruby"))
    .returning(id)
    .get_result(conn)
```

## 3-way join
```
select * from tables
inner join orders on orders.table_id = tables.table_id
inner join order_items on orders.order_id = order_items.order_id
;

```