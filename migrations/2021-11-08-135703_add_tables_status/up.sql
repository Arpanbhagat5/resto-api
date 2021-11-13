-- Your SQL goes here
ALTER TABLE tables
ADD COLUMN IF NOT EXISTS is_table_occupied BOOLEAN DEFAULT False NOT NULL