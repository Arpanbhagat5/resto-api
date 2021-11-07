-- Your SQL goes here
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    prep_time INTEGER,
    order_id SERIAL REFERENCES orders(id) ON DELETE CASCADE,
    item_id SERIAL REFERENCES menu(id),
    status_id SERIAL REFERENCES status(id)
)