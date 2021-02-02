#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::data::account::{Account, NewAccount};

pub mod data;
pub mod database_handler;
pub mod schema;

pub fn establish_database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_account<'a>(
    connection: &PgConnection,
    username: &'a str,
    password_hash: &'a str,
) -> Account {
    use schema::accounts;

    let new_account = NewAccount {
        username: username,
        password_hash: password_hash,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(connection)
        .expect("Error saving new account")
}
