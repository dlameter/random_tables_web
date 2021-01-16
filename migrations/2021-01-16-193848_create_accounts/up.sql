-- Your SQL goes here
CREATE TABLE accounts (
    account_id serial PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,
    password_hash varchar(128) NOT NULL
);