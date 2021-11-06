## resto-api

## setup diesel cli
https://github.com/diesel-rs/diesel/tree/master/diesel_cli
cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"

## setup DB env var
echo DATABASE_URL=:postgres//<USERNAME>:<PASSWORD>@localhost/<DB_NAME> > .env
echo DATABASE_URL=postgres://apiuser:password@localhost/resto-api-db > .env

## create migrations and setup DB
diesel setup
diesel migration generate create_menu_items

## run migrations
diesel migration run
Create table in DB, based on up.sql and creates schema.rs definition too

## revert migrations
diesel migration revert

## redo migration
diesel migration redo (down + up)

## print-schema
Print table definitions for database schema
