-- This file should undo anything in `up.sql`
ALTER TABLE tables
DROP COLUMN IF EXISTS is_table_occupied