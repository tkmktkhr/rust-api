-- Your SQL goes here
ALTER TABLE users
  MODIFY COLUMN id INTEGER UNSIGNED,
  MODIFY COLUMN email VARCHAR(64) NOT NULL;