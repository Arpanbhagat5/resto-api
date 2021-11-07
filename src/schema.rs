table! {
    menu (id) {
        id -> Int4,
        item -> Varchar,
        calories -> Int4,
    }
}

table! {
    order_items (id) {
        id -> Int4,
        prep_time -> Nullable<Int4>,
        order_id -> Int4,
        item_id -> Int4,
        status_id -> Int4,
    }
}

table! {
    orders (id) {
        id -> Int4,
        table_id -> Int4,
    }
}

table! {
    status (id) {
        id -> Int4,
        description -> Varchar,
    }
}

table! {
    tables (id) {
        id -> Int4,
        table_name -> Varchar,
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
