pub const ACCOUNT_TABLE_NAME: &str = "account";
pub const COLUMN_ACCOUNT_ID: &str = "account_id";
pub const COLUMN_ACCOUNT_NAME: &str = "username";
pub const COLUMN_ACCOUNT_PASSWORD: &str = "password_hash";

#[derive(Clone, Debug)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub password: String,
}
