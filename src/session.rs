use crate::data;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct Session {
    connection: Option<String>,
    id: Option<i32>,
    account: Option<data::account::Account>,
}

impl Session {}

pub fn pg_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = ConnectionManager::new(database_url);
    return Pool::new(connection_manager).expect("Failed to establish database pool");
}
