use postgres::{Client, NoTls};

use crate::data::{account, random_table};

const COLUMN_ACCOUNT_ID: &str = "account_id";
const COLUMN_ACCOUNT_NAME: &str = "username";
const COLUMN_ACCOUNT_PASSWORD: &str = "password_hash";
const COLUMN_TABLE_ID: &str = "id";
const COLUMN_TABLE_NAME: &str = "name";
const COLUMN_TABLE_CREATED_BY: &str = "created_by";

pub struct DatabaseHandler {
    connection: postgres::Client
}

impl DatabaseHandler {
    pub fn new() -> Result<DatabaseHandler, String> {
        match Client::connect("host=localhost dbname=random_tables user=postgres password=postgres", NoTls) {
            Ok(connection) => {
                Ok(DatabaseHandler {
                    connection
                })
            },
            Err(e) => Err(format!("Failed to connect to database with error: {}", e)),
        }
    }

    pub fn create_account(&mut self, account: &account::Account) -> Result<(), String> {
        // TODO hash password before sending to database
        match self.connection.execute("INSERT INTO account (username, password_hash) VALUES ($1, $2)", &[&account.name, &account.password]) {
            Ok(_) => Ok(()),
            Err(e) => return Err(format!("Failed to create user with error: {}", e)),
        }
    }

    pub fn find_account_by_id(&mut self, id: &i32) -> Option<account::Account> {
        match self.connection.query_one("SELECT * FROM account WHERE account_id = $1", &[id]) {
            Ok(row) => match DatabaseHandler::row_to_account(&row) {
                Ok(account) => Some(account),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn find_account_by_name(&mut self, name: &String) -> Option<account::Account> {
        match self.connection.query_one("SELECT * FROM account WHERE username = $1", &[name]) {
            Ok(row) => match DatabaseHandler::row_to_account(&row) {
                Ok(account) => Some(account),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
    
    fn row_to_account(row: &postgres::row::Row) -> Result<account::Account, String> {
        let id: i32 = match row.try_get(COLUMN_ACCOUNT_ID) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_ACCOUNT_ID, error)),
        };
        let name: String = match row.try_get(COLUMN_ACCOUNT_NAME) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_ACCOUNT_NAME, error)),
        };
        let password: String = match row.try_get(COLUMN_ACCOUNT_PASSWORD) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_ACCOUNT_PASSWORD, error)),
        };

        Ok(account::Account {
            id,
            name,
            password,
        })
    }

    pub fn create_table(&mut self, table: &random_table::Table) -> Result<(), String> {
        match self.connection.query("INSERT INTO random_table (created_by, name) VALUES ($1, $2)", &[&table.created_by, &table.name]) {
            Ok(value) => Ok(()), // Also create element entries
            Err(error) => Err(format!("Failed to create random_table entry with error: {}", error)),
        }
    }

    fn row_to_table(row: &postgres::row::Row) -> Result<random_table::Table, String> {
        let id: i32 = match row.try_get(COLUMN_TABLE_ID) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_TABLE_ID, error))
        };
        let created_by: i32 = match row.try_get(COLUMN_TABLE_CREATED_BY) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_TABLE_CREATED_BY, error))
        };
        let name: String = match row.try_get(COLUMN_TABLE_NAME) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_TABLE_NAME, error))
        };

        Ok(random_table::Table {
            id,
            created_by,
            name,
            elements: Vec::new(),
        })
    }
}
