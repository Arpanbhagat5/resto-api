-- Your SQL goes here
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    prep_time INTEGER,
    order_id SERIAL REFERENCES orders(order_id) ON DELETE CASCADE,
    item_id SERIAL REFERENCES menu(item_id),
    status_id SERIAL REFERENCES status(status_id)
)