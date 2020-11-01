use postgres::{Client, NoTls};

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

    pub fn create_user(&mut self, username: &String, password: &String) -> Result<(), String> {
        // TODO hash password before sending to database
        match self.connection.execute("INSERT INTO account (username, password_hash) VALUES ($1, $2)", &[username, password]) {
            Ok(_) => Ok(()),
            Err(e) => return Err(format!("Failed to create user with error: {}", e)),
        }
    }
}
