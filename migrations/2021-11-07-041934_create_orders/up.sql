-- Your SQL goes here
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    table_id SERIAL REFERENCES tables(id) ON DELETE SET NULL
)