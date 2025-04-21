-- This file should undo anything in `up.sql`
-- For PostgreSQL:
ALTER TABLE posts ALTER COLUMN created_by SET NOT NULL;