use crate::data;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Session {
    connection: PooledPg,
    id: Option<i32>,
    account: Option<data::account::Account>,
}

impl Session {
    pub fn from_key(conn: PooledPg, key: Option<&str>) -> Self {
        use crate::schema::accounts::dsl as a;
        use crate::schema::web_sessions::dsl as s;

        let (id, account) = key
            .and_then(|key| {
                a::accounts
                    .inner_join(s::web_sessions)
                    .select((s::id, (a::id, a::username)))
                    .filter(s::cookie.eq(&key))
                    .first::<(i32, data::account::Account)>(&conn)
                    .ok()
            })
            .map(|(id, account)| (Some(id), Some(account)))
            .unwrap_or((None, None));

        Session {
            connection: conn,
            id,
            account,
        }
    }
}

pub fn pg_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = ConnectionManager::new(database_url);
    return Pool::new(connection_manager).expect("Failed to establish database pool");
}
