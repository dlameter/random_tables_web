#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::Connection;
use diesel::pg::PgConnection;
use std::env;

pub mod schema;

pub mod data;
pub mod database_handler;

pub fn establish_database_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}