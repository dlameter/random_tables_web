use postgres::{Client, NoTls};

use crate::data::account;

const COLUMN_ID: &str = "account_id";
const COLUMN_NAME: &str = "username";
const COLUMN_PASSWORD: &str = "password_hash";

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
    
    fn row_to_account(row: &postgres::row::Row) -> Result<account::Account, String> {
        let id: i32 = match row.try_get(COLUMN_ID) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_ID, error)),
        };
        let name: String = match row.try_get(COLUMN_NAME) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_NAME, error)),
        };
        let password: String = match row.try_get(COLUMN_PASSWORD) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", COLUMN_PASSWORD, error)),
        };

        Ok(account::Account {
            id,
            name,
            password,
        })
    }
}
