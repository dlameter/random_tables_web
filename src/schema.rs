table! {
    accounts (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

table! {
    random_table_elements (index, table_id) {
        index -> Int4,
        table_id -> Int4,
        text -> Varchar,
    }
}

table! {
    random_tables (id) {
        id -> Int4,
        created_by -> Int4,
        name -> Varchar,
    }
}

table! {
    web_sessions (id) {
        id -> Int4,
        cookie -> Varchar,
        account_id -> Int4,
    }
}

joinable!(random_table_elements -> random_tables (table_id));
joinable!(random_tables -> accounts (created_by));
joinable!(web_sessions -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    random_table_elements,
    random_tables,
    web_sessions,
);
