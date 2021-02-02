#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::data::account::{Account, NewAccount};

pub mod data;
pub mod schema;
pub mod session;

pub fn establish_database_connection() -> session::PooledPg {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    session::pg_pool(&database_url).get().unwrap()
}

pub fn create_account<'a>(
    connection: &PgConnection,
    username: &'a str,
    password_hash: &'a str,
) -> Account {
    use schema::accounts as a;

    let new_account = NewAccount {
        username: username,
        password_hash: password_hash,
    };

    let result = diesel::insert_into(a::table)
        .values(&new_account)
        .returning((a::dsl::id, a::dsl::username))
        .get_result::<Account>(connection)
        .expect("Error saving new account");

    result
}
