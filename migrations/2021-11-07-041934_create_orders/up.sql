-- Your SQL goes here
CREATE TABLE orders (
    order_id SERIAL PRIMARY KEY,
    table_id SERIAL REFERENCES tables(table_id) ON DELETE SET NULL
)