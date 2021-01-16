-- Your SQL goes here
CREATE TABLE accounts (
    id serial PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE,
    password_hash varchar(128) NOT NULL
);