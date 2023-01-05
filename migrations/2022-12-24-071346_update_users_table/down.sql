-- This file should undo anything in `up.sql`
ALTER TABLE users
  MODIFY COLUMN id INTEGER SIGNED,
  MODIFY COLUMN email VARCHAR(64) NULL;
