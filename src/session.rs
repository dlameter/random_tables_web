use crate::data::account::Account;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Session {
    connection: PooledPg,
    id: Option<i32>,
    account: Option<Account>,
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
                    .first::<(i32, Account)>(&conn)
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

    pub fn authenticate(&mut self, username: &str, password: &str) -> Option<String> {
        if let Some(account) = Account::authenticate(&self.connection, username, password) {
            // Create session key
            let session_key = random_key(48);

            // store info in DB
            use crate::schema::web_sessions::dsl::*;
            let result = diesel::insert_into(web_sessions)
                .values((account_id.eq(account.id), cookie.eq(&session_key)))
                .returning(id)
                .get_results(&self.connection);

            // store info in Session object
            if let Ok([session_id]) = result.as_ref().map(|value| &**value) {
                self.account = Some(account);
                self.id = Some(*session_id);
                return Some(session_key);
            } else {
                println!(
                    "Failed to create session id for {}: {:?}",
                    account.username, result
                );
            }
        }
        None
    }
}

fn random_key(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn pg_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = ConnectionManager::new(database_url);
    return Pool::new(connection_manager).expect("Failed to establish database pool");
}
