use postgres::{Client, NoTls};
use postgres::error::Error as PgError;

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
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Failed to create random_table entry with error: {}", error)),
        }
    }

    pub fn find_table_by_id(&mut self, id: &i32) -> Option<random_table::Table> {
        match self.connection.query_one("SELECT * FROM random_table WHERE id = $1", &[id]) {
            Ok(row) => match DatabaseHandler::row_to_table(&row) {
                Ok(table) => Some(table),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn list_tables_by_creator_id(&mut self, creator_id: &i32) -> Vec<random_table::Table> {
        let mut results = Vec::new();

        match self.connection.query("SELECT * FROM random_table WHERE created_by = $1", &[creator_id]) {
            Ok(rows) => {
                for row in rows {
                    if let Ok(table) = DatabaseHandler::row_to_table(&row) {
                        results.push(table);
                    }
                }
            },
            Err(_) => return results,
        }

        results
    }

    pub fn find_table_elements_by_table_id(&mut self, table_id: &i32) -> Option<Vec<String>> {
        if let Ok(rows) = self.connection.query("SELECT * FROM random_table_element WHERE table_id = $1", &[table_id]) {
            if let Ok(elements_option) = DatabaseHandler::row_vec_to_element_vec(&rows) {
                return elements_option;
            }
        }
        None
    }

    pub fn delete_table_and_elements(&mut self, table: &random_table::Table) -> Result<random_table::Table, PgError> {
        let mut transaction = self.connection.transaction()?;

        let table_result = DatabaseHandler::delete_table_transaction(&mut transaction, table);
        let elements_result = DatabaseHandler::delete_table_elements_transaction(&mut transaction, table);

        if let Ok(mut table) = table_result {
            table.elements = match elements_result {
                Ok(elements) => elements,
                Err(_) => None,
            };

            transaction.commit()?;
            Ok(table)
        }
        else {
            transaction.rollback()?;
            table_result
        }
    }

    pub fn delete_table(&mut self, table: &random_table::Table) -> Result<random_table::Table, PgError> {
        let mut transaction = self.connection.transaction()?;

        match DatabaseHandler::delete_table_transaction(&mut transaction, table) {
            Ok(table) => {
                transaction.commit()?;
                Ok(table)
            },
            Err(error) => {
                transaction.rollback()?;
                Err(error)
            },
        }
    }

    fn delete_table_transaction(transaction: &mut postgres::Transaction, table: &random_table::Table) -> Result<random_table::Table, PgError> {
        let row = transaction.query_one("DELETE FROM random_table WHERE id = $1 AND created_by = $2 RETURNING *", &[&table.id, &table.created_by])?;
        DatabaseHandler::row_to_table(&row)
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

    pub fn delete_table_elements(&mut self, table: &random_table::Table) -> Result<Option<Vec<String>>, PgError> {
        let mut transaction = self.connection.transaction()?;
        
        match DatabaseHandler::delete_table_elements_transaction(&mut transaction, table) {
            Ok(elements) => {
                transaction.commit()?;
                Ok(elements)
            },
            Err(error) => {
                transaction.rollback()?;
                Err(error)
            },
        }
    }

    fn delete_table_elements_transaction(transaction: &mut postgres::Transaction, table: &random_table::Table) -> Result<Option<Vec<String>>, PgError> {
        let rows = transaction.query("DELETE FROM random_table_element WHERE table_id = $1 RETURNING *", &[&table.id])?;
        DatabaseHandler::row_vec_to_element_vec(&rows)
    }

    fn row_to_table(row: &postgres::row::Row) -> Result<random_table::Table, PgError> {
        let id: i32 = row.try_get(random_table::COLUMN_TABLE_ID)?;
        let created_by: i32 = row.try_get(random_table::COLUMN_TABLE_CREATED_BY)?;
        let name: String = row.try_get(random_table::COLUMN_TABLE_NAME)?;

        Ok(random_table::Table {
            id,
            created_by,
            name,
            elements: None,
        })
    }

    fn row_vec_to_element_vec(rows: &Vec<postgres::row::Row>) -> Result<Option<Vec<String>>, PgError> {
        if rows.is_empty() {
            return Ok(None);
        }

        let mut elements: Vec<String> = Vec::new();

        for row in rows {
            let text: String = row.try_get(random_table::COLUMN_TABLE_ELEMENT_TEXT)?;
            elements.push(text);
        }

        Ok(Some(elements))
    }
}
