-- Your SQL goes here
CREATE TABLE menu (
    id SERIAL PRIMARY KEY,
    item VARCHAR NOT NULL,
    calories INTEGER NOT NULL
)