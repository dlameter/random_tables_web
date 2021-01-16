extern crate random_tables_web;
extern crate diesel;

use self::random_tables_web::*;
use self::diesel::prelude::*;

fn main() {
    use random_tables_web::schema::accounts::dsl::*;

    let connection = establish_database_connection();
    let results = accounts.order(id.asc())
        .limit(5)
        .load::<data::account::Account>(&connection)
        .expect("Error loading accounts");
    
    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{} {} {}", account.id, account.name, account.password_hash);
    }
}