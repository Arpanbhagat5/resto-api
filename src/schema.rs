table! {
    menu (item_id) {
        item_id -> Int4,
        item -> Varchar,
        calories -> Int4,
    }
}

table! {
    order_items (id) {
        id -> Int4,
        prep_time -> Int4,
        order_id -> Int4,
        item_id -> Int4,
        status_id -> Int4,
    }
}

table! {
    orders (order_id) {
        order_id -> Int4,
        table_id -> Int4,
    }
}

table! {
    status (status_id) {
        status_id -> Int4,
        description -> Varchar,
    }
}

table! {
    tables (table_id) {
        table_id -> Int4,
        table_name -> Varchar,
        is_occupied -> Bool,
    }
}

joinable!(order_items -> menu (item_id));
joinable!(order_items -> orders (order_id));
joinable!(order_items -> status (status_id));
joinable!(orders -> tables (table_id));

allow_tables_to_appear_in_same_query!(
    menu,
    order_items,
    orders,
    status,
    tables,
);
