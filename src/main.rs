use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use warp::{http::Uri, Filter, filters::BoxedFilter};
use liquid::object;

mod templating;
use templating::Templator;

mod database_handler;
use database_handler::DatabaseHandler;

mod data;

type SharedDatabaseHandler = Arc<Mutex<DatabaseHandler>>;

#[tokio::main]
async fn main() {
    let templator = Templator::new("./_layout/".to_string(), "./_includes/".to_string());
    let templator = Arc::new(templator);
    
    let template_file = move |(file_path, globals)| warp::reply::html(templator.clone().render_file(file_path, &globals));
    let template_file = Arc::new(Mutex::new(template_file));

    let template_file_clone = Arc::clone(&template_file);
    let index = warp::get().and(warp::path("index.html"))
        .map(|| (Path::new("index.html"), object!({})))
        .map(move |args| template_file_clone.lock().unwrap()(args));
    let index_redirect = warp::get().and(warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html"))));

    let static_files = warp::get().and(warp::path("static").and(warp::fs::dir("static")));

    let handler = match DatabaseHandler::new() {
        Ok(c) => c,
        Err(e) => panic!(format!("Failed to create database handler with error: {}", e)),
    };
    let handler = Arc::new(Mutex::new(handler));

    let handler_clone = Arc::clone(&handler);
    let template_file_clone = Arc::clone(&template_file);
    let account_by_id = warp::get().and(warp::path!("id" / i32))
        .and(warp::path::end())
        .map(move |id| {
            match handler_clone.lock().unwrap().find_account_by_id(&id) {
                Some(account) => {
                    let globals = object!({
                        "username": account.name,
                        "userid": account.id,
                    });
                    template_file_clone.lock().unwrap()((Path::new("account.html"), globals))
                },
                None => warp::reply::html(format!("Could not find user with id {}", id)),
            }
        });

    let account_by_name = build_account_by_name_filter(&handler);

    let handler_clone = Arc::clone(&handler);
    let tables_by_account_id = warp::get().and(warp::path!("id" / i32 / "tables"))
        .and(warp::path::end())
        .map(move |account_id| {
            let tables = handler_clone.lock().unwrap().list_tables_by_creator_id(&account_id);
            format!("{:?}", tables)
        });

    let delete_account = build_delete_account_filter(&handler);

    let create_account = build_create_account_filter(&handler);

    let handler_clone = Arc::clone(&handler);
    let update_account = warp::post().and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .map(move |json_map: HashMap<String, String>| {
            if let (Some(id), Some(name), Some(password)) = (json_map.get("id"), json_map.get("name"), json_map.get("password")) {
                if let Ok(id) = i32::from_str_radix(id, 10) {
                    let updated_account = data::account::Account {
                        id,
                        name: name.clone(),
                        password: password.clone(),
                    };
                    
                    match handler_clone.lock().unwrap().update_account(&updated_account) {
                        Ok(account) => return format!("Updated account to {:?}", account),
                        Err(error) => return format!("Failed to update account with error: {}", error),
                    }
                }
            }
            format!("Failed to update account, name, password, not specified or id not an int.")
        });

    let accounts_endpoint = warp::path("account")
        .and(
            account_by_id
            .or(account_by_name)
            .or(tables_by_account_id)
            .or(delete_account)
            .or(create_account)
            .or(update_account)
        );

    let routes = accounts_endpoint.or(index.or(index_redirect).or(static_files));

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}

fn build_account_by_name_filter(handler: &SharedDatabaseHandler) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::get().and(warp::path!("name" / String)).and(warp::path::end())
        .map(move |name| {
            match handler_clone.lock().unwrap().find_account_by_name(&name) {
                Some(account) => format!("{:?}", account),
                None => format!("Could not find user with name {}", name),
            }
        }).boxed()
}

fn build_create_account_filter(handler: &SharedDatabaseHandler) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::post().and(warp::path("create")).and(warp::path::end()).and(warp::body::content_length_limit(1024 * 32)).and(warp::body::json())
        .map(move |json_map: HashMap<String, String>| {
            if let (Some(username), Some(password)) = (json_map.get("username"), json_map.get("password")) {
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
            format!("Failed to create account, username or password not supplied.")
        }).boxed()
}

fn build_delete_account_filter(handler: &SharedDatabaseHandler) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::get().and(warp::path!("id" / i32 / "delete")).and(warp::path::end())
        .map(move |id| {
            match handler_clone.lock().unwrap().delete_account(&id) {
                Ok(account) => format!("{:?}", account),
                Err(error) => format!("Failed to delete account id {} with error: {}", id, error),
            }
        }).boxed()
}
