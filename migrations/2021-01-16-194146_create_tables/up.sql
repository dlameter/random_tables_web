-- Your SQL goes here
CREATE TABLE random_tables (
    id serial PRIMARY KEY,
    created_by int NOT NULL,
    name varchar(128) NOT NULL,
    FOREIGN KEY (created_by) REFERENCES accounts (id)
);

CREATE TABLE random_table_elements (
    index int NOT NULL,
    table_id int NOT NULL,
    text varchar(256) NOT NULL,
    FOREIGN KEY (table_id) REFERENCES random_tables (id),
    CONSTRAINT random_table_element_pk PRIMARY KEY(index, table_id)
);