use postgres::{Client, NoTls};

use crate::data::{account, random_table};

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
        let id: i32 = match row.try_get(account::COLUMN_ACCOUNT_ID) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", account::COLUMN_ACCOUNT_ID, error)),
        };
        let name: String = match row.try_get(account::COLUMN_ACCOUNT_NAME) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", account::COLUMN_ACCOUNT_NAME, error)),
        };
        let password: String = match row.try_get(account::COLUMN_ACCOUNT_PASSWORD) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", account::COLUMN_ACCOUNT_PASSWORD, error)),
        };

        Ok(account::Account {
            id,
            name,
            password,
        })
    }

    pub fn create_table(&mut self, table: &random_table::Table) -> Result<(), String> {
        match self.connection.query("INSERT INTO random_table (created_by, name) VALUES ($1, $2)", &[&table.created_by, &table.name]) {
            Ok(value) => Ok(()),
            Err(error) => Err(format!("Failed to create random_table entry with error: {}", error)),
        }
    }

    pub fn delete_table(&mut self, table: &random_table::Table) -> Result<random_table::Table, String> {
        let row = self.connection.query_one("DELETE FROM random_table WHERE id = $1 AND created_by = $2 RETURNING *", &[&table.id, &table.created_by])
            .map_err(|error| format!("Failed to delete table with error: {}", error))?;

        Ok(DatabaseHandler::row_to_table(&row)?)
    }

    pub fn create_table_elements(&mut self, table: &random_table::Table) -> Result<(), String> {
        if let Some(ref elements) = table.elements {
            let statement = self.connection.prepare("INSERT INTO random_table_element (table_id, index, text) VALUES ($1, $2, $3)")
                .map_err(|error| format!("Failed to create statment with error: {}", error))?;

            for (index, value) in elements.iter().enumerate() {
                self.connection.execute(&statement, &[&table.id, &(index as i32), value])
                    .map_err(|error| format!("Failed to insert row in random_table_element with error: {}", error))?;
            }

            Ok(())
        }
        else {
            return Err("Tried to create elements of table that has no elements.".to_string());
        }
    }

    pub fn delete_table_elements(&mut self, table: &random_table::Table) -> Result<Vec<String>, String> {
        let rows = self.connection.query("DELETE FROM random_table_element WHERE table_id = $1 RETURNING *", &[&table.id])
            .map_err(|error| format!("Failed to delete rows from random_table_element with error: {}", error))?;

        Ok(DatabaseHandler::row_vec_to_element_vec(&rows)?)
    }

    fn row_to_table(row: &postgres::row::Row) -> Result<random_table::Table, String> {
        let id: i32 = match row.try_get(random_table::COLUMN_TABLE_ID) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", random_table::COLUMN_TABLE_ID, error))
        };
        let created_by: i32 = match row.try_get(random_table::COLUMN_TABLE_CREATED_BY) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", random_table::COLUMN_TABLE_CREATED_BY, error))
        };
        let name: String = match row.try_get(random_table::COLUMN_TABLE_NAME) {
            Ok(value) => value,
            Err(error) => return Err(format!("Failed to get column {} with error: {}", random_table::COLUMN_TABLE_NAME, error))
        };

        Ok(random_table::Table {
            id,
            created_by,
            name,
            elements: None,
        })
    }

    fn row_vec_to_element_vec(rows: &Vec<postgres::row::Row>) -> Result<Vec<String>, String> {
        let mut elements: Vec<String> = Vec::new();

        for row in rows {
            let text: String = match row.try_get(random_table::COLUMN_TABLE_ELEMENT_TEXT) {
                Ok(value) => value,
                Err(error) => return Err(format!("Failed to get column {} with error: {}", random_table::COLUMN_TABLE_ELEMENT_TEXT, error)),
            };

            elements.push(text);
        }

        Ok(elements)
    }
}
