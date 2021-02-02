extern crate diesel;
extern crate random_tables_web;

use self::diesel::prelude::*;
use self::random_tables_web::*;

fn main() {
    use random_tables_web::schema::accounts::dsl::*;

    let connection = establish_database_connection();
    let results = accounts
        .order(id.asc())
        .select((id, username))
        .limit(5)
        .load::<data::account::Account>(&connection)
        .expect("Error loading accounts");
    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{} {}", account.id, account.username);
    }
}
