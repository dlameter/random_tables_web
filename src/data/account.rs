use serde::{Deserialize, Serialize};

pub const ACCOUNT_TABLE_NAME: &str = "account";
pub const COLUMN_ACCOUNT_ID: &str = "account_id";
pub const COLUMN_ACCOUNT_NAME: &str = "username";
pub const COLUMN_ACCOUNT_PASSWORD: &str = "password_hash";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
}
