-- Your SQL goes here
CREATE TABLE users
(
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64),
    email VARCHAR(64)
);