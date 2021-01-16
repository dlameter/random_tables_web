-- Your SQL goes here
CREATE TABLE web_sessions {
    id serial PRIMARY KEY,
    cookie varchar NOT NULL,
    user_id int NOT NULL REFERENCES accounts (id)
};