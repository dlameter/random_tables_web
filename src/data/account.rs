use crate::schema::accounts;
use crate::session::PooledPg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub const ACCOUNT_TABLE_NAME: &str = "account";
pub const COLUMN_ACCOUNT_ID: &str = "id";
pub const COLUMN_ACCOUNT_NAME: &str = "username";
pub const COLUMN_ACCOUNT_PASSWORD: &str = "password_hash";

#[derive(Clone, Debug, Deserialize, Serialize, Queryable)]
pub struct Account {
    pub id: i32,
    pub username: String,
}

impl Account {
    pub fn authenticate(conn: &PooledPg, username: &str, password: &str) -> Option<Self> {
        use crate::schema::accounts::dsl as a;

        let (password_hash, account_result) = a::accounts
            .filter(a::username.eq(username))
            .select((a::password_hash, (a::id, a::username)))
            .first::<(String, Self)>(conn)
            .ok()
            .map(|(pass_hash, account_result)| (Some(pass_hash), Some(account_result)))
            .unwrap_or((None, None));

        if let Some(password_hash) = password_hash {
            if Self::test_password(password, &password_hash) {
                return account_result;
            }
        }
        None
    }

    fn test_password(password: &str, password_hash: &str) -> bool {
        password == password_hash
    }
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}
