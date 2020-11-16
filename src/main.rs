use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use warp::{http::Uri, Filter};
use liquid::object;

mod templating;
use templating::Templator;

mod database_handler;
use database_handler::DatabaseHandler;

mod data;

#[tokio::main]
async fn main() {
    let handler = match DatabaseHandler::new() {
        Ok(c) => c,
        Err(e) => panic!(format!("Failed to create database handler with error: {}", e)),
    };
    let handler = Arc::new(Mutex::new(handler));
    let handler_clone = Arc::clone(&handler);
    let account_by_id = warp::get().and(warp::path!("id" / i32))
        .and(warp::path::end())
        .map(move |id| {
            match handler_clone.lock().unwrap().find_account_by_id(&id) {
                Some(account) => format!("{:?}", account),
                None => format!("Could not find user with id {}", id),
            }
        });

    let handler_clone = Arc::clone(&handler);
    let account_by_name = warp::get().and(warp::path!("name" / String))
        .and(warp::path::end())
        .map(move |name| {
            match handler_clone.lock().unwrap().find_account_by_name(&name) {
                Some(account) => format!("{:?}", account),
                None => format!("Could not find user with name {}", name),
            }
        });

    let handler_clone = Arc::clone(&handler);
    let tables_by_account_id = warp::get().and(warp::path!("id" / i32 / "tables"))
        .and(warp::path::end())
        .map(move |account_id| {
            let tables = handler_clone.lock().unwrap().list_tables_by_creator_id(&account_id);
            format!("{:?}", tables)
        });

    let handler_clone = Arc::clone(&handler);
    let delete_account = warp::get().and(warp::path!("id" / i32 / "delete"))
        .and(warp::path::end())
        .map(move |id| {
            match handler_clone.lock().unwrap().delete_account(&id) {
                Ok(account) => format!("{:?}", account),
                Err(error) => format!("Failed to delete account id {} with error: {}", id, error),
            }
        });

    let handler_clone = Arc::clone(&handler);
    let create_account = warp::post().and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .map(move |json_map: HashMap<String, String>| {
            if let Some(username) = json_map.get("username") {
            if let Some(password) = json_map.get("password") {
                let new_account = data::account::Account {
                    id: 0,
                    name: username.clone(),
                    password: password.clone(),
                };

                match handler_clone.lock().unwrap().create_account(&new_account) {
                    Ok(_) => return "Account created".to_string(),
                    Err(error) => return format!("Failed to create account with error: {}", error),
                }
            }
            }
            format!("Failed to create account, username or password not supplied.")
        });

    let accounts_endpoint = warp::path("account")
        .and(
            account_by_id
            .or(account_by_name)
            .or(tables_by_account_id)
            .or(delete_account)
            .or(create_account)
        );

    let templator = Templator::new("./_layout/".to_string(), "./_includes/".to_string());
    let templator = Arc::new(templator);
    
    let template_file = move |file_path| warp::reply::html(templator.clone().render_file(file_path, &object!({})));

    let index = warp::get().and(warp::path("index.html"))
        .map(|| Path::new("index.html"))
        .map(template_file);
    let index_redirect = warp::get().and(warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html"))));

    let static_files = warp::get().and(warp::path("static").and(warp::fs::dir("static")));

    let routes = accounts_endpoint.or(index.or(index_redirect).or(static_files));

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}
