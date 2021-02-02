use crate::schema::accounts;
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

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}
