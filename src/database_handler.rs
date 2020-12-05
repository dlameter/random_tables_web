use postgres::{Client, NoTls};
use postgres::error::Error as PgError;

use crate::data::{account, random_table};

pub struct DatabaseConfig {
    host: String,
    dbname: String,
    user: String,
    password: String,
}

impl DatabaseConfig {
    pub fn new(host: String, dbname: String, user: String, password: String) -> Self {
        Self {
            host,
            dbname,
            user,
            password,
        }
    }
}

pub struct DatabaseHandler {
    connection: postgres::Client
}

impl DatabaseHandler {
    pub fn new(database_config: &DatabaseConfig) -> Result<DatabaseHandler, PgError> {
        let connect_string = format!("host={} dbname={} user={} password={}", database_config.host.as_str(), database_config.dbname.as_str(), database_config.user.as_str(), database_config.password.as_str());
        let connection = Client::connect(connect_string.as_str(), NoTls)?;
        Ok(DatabaseHandler {
            connection
        })
    }

    pub fn create_account(&mut self, account: &account::Account) -> Result<account::Account, PgError> {
        // TODO hash password before sending to database
        let query_string = format!("INSERT INTO {} ({}, {}) VALUES ($1, $2) RETURNING *", account::ACCOUNT_TABLE_NAME, account::COLUMN_ACCOUNT_NAME, account::COLUMN_ACCOUNT_PASSWORD);
        let row = self.connection.query_one(query_string.as_str(), &[&account.name, &account.password])?;
        DatabaseHandler::row_to_account(&row)
    }

    pub fn find_account_by_id(&mut self, id: &i32) -> Option<account::Account> {
        let query_string = format!("SELECT * FROM {} WHERE {} = $1", account::ACCOUNT_TABLE_NAME, account::COLUMN_ACCOUNT_ID);
        match self.connection.query_one(query_string.as_str(), &[id]) {
            Ok(row) => match DatabaseHandler::row_to_account(&row) {
                Ok(account) => Some(account),
                Err(error) => {
                    println!("Failed to find account by id with error: {}", error);
                    None
                },
            },
            Err(error) => {
                println!("Failed to find account by id with error: {}", error);
                None
            },
        }
    }

    pub fn find_account_by_name(&mut self, name: &String) -> Option<account::Account> {
        let query_string = format!("SELECT * FROM {} WHERE {} = $1", account::ACCOUNT_TABLE_NAME, account::COLUMN_ACCOUNT_NAME);
        match self.connection.query_one(query_string.as_str(), &[name]) {
            Ok(row) => match DatabaseHandler::row_to_account(&row) {
                Ok(account) => Some(account),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn update_account(&mut self, account: &account::Account) -> Result<account::Account, PgError> {
        let query_string = format!("UPDATE {} SET {} = $1, {} = $2 WHERE {} = $3 RETURNING *", account::ACCOUNT_TABLE_NAME, account::COLUMN_ACCOUNT_NAME, account::COLUMN_ACCOUNT_PASSWORD, account::COLUMN_ACCOUNT_ID);
        let row = self.connection.query_one(query_string.as_str(), &[&account.name, &account.password, &account.id])?;
        DatabaseHandler::row_to_account(&row)
    }

    pub fn delete_account(&mut self, id: &i32) -> Result<account::Account, PgError> {
        let query_string = format!("DELETE FROM {} WHERE {} = $1 RETURNING *", account::ACCOUNT_TABLE_NAME, account::COLUMN_ACCOUNT_ID);
        let row = self.connection.query_one(query_string.as_str(), &[id])?;
        DatabaseHandler::row_to_account(&row)
    }
    
    fn row_to_account(row: &postgres::row::Row) -> Result<account::Account, PgError> {
        let id: i32 = row.try_get(account::COLUMN_ACCOUNT_ID)?;
        let name: String = row.try_get(account::COLUMN_ACCOUNT_NAME)?;
        let password: String = row.try_get(account::COLUMN_ACCOUNT_PASSWORD)?;

        Ok(account::Account {
            id: Some(id),
            name,
            password,
        })
    }

    pub fn create_table(&mut self, table: &random_table::Table) -> Result<random_table::Table, PgError> {
        let query_string = format!("INSERT INTO {} ({}, {}) VALUES ($1, $2)", random_table::TABLE_TABLE_NAME, random_table::COLUMN_TABLE_CREATED_BY, random_table::COLUMN_TABLE_NAME);
        let row = self.connection.query_one(query_string.as_str(), &[&table.created_by, &table.name])?;
        DatabaseHandler::row_to_table(&row)
    }

    pub fn find_table_by_id(&mut self, id: &i32) -> Option<random_table::Table> {
        let query_string = format!("SELECT * FROM {} WHERE {} = $1", random_table::TABLE_TABLE_NAME, random_table::COLUMN_TABLE_ID);
        match self.connection.query_one(query_string.as_str(), &[id]) {
            Ok(row) => match DatabaseHandler::row_to_table(&row) {
                Ok(table) => Some(table),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn list_tables_by_creator_id(&mut self, creator_id: &i32) -> Vec<random_table::Table> {
        let mut results = Vec::new();

        let query_string = format!("SELECT * FROM {} WHERE {} = $1", random_table::TABLE_TABLE_NAME, random_table::COLUMN_TABLE_CREATED_BY);
        match self.connection.query(query_string.as_str(), &[creator_id]) {
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

    pub fn fill_table_with_elements(&mut self, table: &mut random_table::Table) {
        table.elements = self.find_table_elements_by_table_id(&table.id);
    }

    pub fn find_table_elements_by_table_id(&mut self, table_id: &i32) -> Option<Vec<String>> {
        let query_string = format!("SELECT * FROM {} WHERE {} = $1", random_table::TABLE_ELEMENT_TABLE_NAME, random_table::COLUMN_TABLE_ELEMENT_TABLE_ID);
        if let Ok(rows) = self.connection.query(query_string.as_str(), &[table_id]) {
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
        let query_string = format!("DELETE FROM {} WHERE {} = $1 AND {} = $2 RETURNING *", random_table::TABLE_TABLE_NAME, random_table::COLUMN_TABLE_ID, random_table::COLUMN_TABLE_CREATED_BY);
        let row = transaction.query_one(query_string.as_str(), &[&table.id, &table.created_by])?;
        DatabaseHandler::row_to_table(&row)
    }

    pub fn create_table_elements(&mut self, table: &random_table::Table) -> Result<(), String> {
        if let Some(ref elements) = table.elements {
            let query_string = format!("INSERT INTO {} ({}, {}, {}) VALUES ($1, $2, $3)", random_table::TABLE_ELEMENT_TABLE_NAME, random_table::COLUMN_TABLE_ELEMENT_TABLE_ID, random_table::COLUMN_TABLE_ELEMENT_INDEX, random_table::COLUMN_TABLE_ELEMENT_TEXT);
            let statement = self.connection.prepare(query_string.as_str())
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
        let query_string = format!("DELETE FROM {} WHERE {} = $1 RETURNING *", random_table::TABLE_ELEMENT_TABLE_NAME, random_table::COLUMN_TABLE_ELEMENT_TABLE_ID);
        let rows = transaction.query(query_string.as_str(), &[&table.id])?;
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
