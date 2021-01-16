extern crate random_tables_web;
extern crate diesel;

use self::random_tables_web::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_database_connection();

    println!("What would you like your account username to be?");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("What would you like your account password to be?");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let account = create_account(&connection, &username, &password);
    println!("Saved account {} with id {}", account.name, account.id);
}