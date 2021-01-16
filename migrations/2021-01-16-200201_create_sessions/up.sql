-- Your SQL goes here
CREATE TABLE web_sessions (
    id serial PRIMARY KEY,
    cookie varchar NOT NULL,
    account_id int NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
);