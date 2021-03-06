extern crate diesel;
extern crate random_tables_web;

use diesel::prelude::*;
use random_tables_web::*;
use std::env::args;
use std::io::stdin;

fn main() {
    use self::schema::accounts::dsl::*;

    let connection = establish_database_connection();

    let account_id = args().nth(1).expect("account id not supplied");
    let account_id: i32 = account_id
        .parse()
        .expect("account id could not be parsed as i32");

    let old_account = accounts
        .filter(id.eq(account_id))
        .select((id, username))
        .load::<data::account::Account>(&connection)
        .expect(&format!("Could not find account with id {}", account_id));
    let old_account = old_account.first().expect("No account found");

    println!(
        "Account {}'s old values, name = {}",
        old_account.id, old_account.username
    );

    println!("What would you like to change the account name to?");
    let mut new_username = String::new();
    stdin().read_line(&mut new_username).unwrap();
    let new_username = new_username.trim();

    println!("What would you like to change the account password to?");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let new_account: data::account::Account = diesel::update(accounts.filter(id.eq(account_id)))
        .set((username.eq(new_username), password_hash.eq(password)))
        .returning((id, username))
        .get_result(&connection)
        .expect(&format!("Failed to update account {}", account_id));

    println!(
        "Account {} updated to name = {}",
        new_account.id, new_account.username
    );
}
