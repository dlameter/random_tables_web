use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use warp::{Filter, filters::BoxedFilter, http::Uri};

use random_tables_web::data;
use random_tables_web::database_handler::DatabaseHandler;
use random_tables_web::PathAndObject;
use random_tables_web::templating::Templator;

type SharedDatabaseHandler = Arc<Mutex<DatabaseHandler>>;

#[tokio::main]
async fn main() {
    let templator = Templator::new("./_layout/".to_string(), "./_includes/".to_string());

    let templator = Arc::new(Mutex::new(templator));

    let templator_clone = templator.clone();
    let index = warp::get()
        .and(warp::path("index.html"))
        .map(|| PathAndObject::default())
        .map(move |pa: PathAndObject| pa.render_file(templator_clone.clone()));

    let index_redirect =
        warp::get().and(warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html"))));

    let static_files = warp::get().and(warp::path("static").and(warp::fs::dir("static")));

    let handler = match DatabaseHandler::new() {
        Ok(c) => c,
        Err(e) => panic!(format!(
            "Failed to create database handler with error: {}",
            e
        )),
    };
    let handler = Arc::new(Mutex::new(handler));

    let accounts_endpoint = build_account_endpoint(&handler, templator.clone());

    let routes = accounts_endpoint.or(index.or(index_redirect).or(static_files));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn build_account_endpoint(
    handler: &SharedDatabaseHandler,
    templator: Arc<Mutex<Templator>>,
) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path("account")
        .and(
            build_account_by_id_filter(handler, templator.clone())
                .or(build_account_by_name_filter(handler))
                .or(build_tables_by_account_id_filter(handler))
                .or(build_delete_account_filter(handler))
                .or(build_create_account_filter(handler))
                .or(build_update_account_filter(handler)),
        )
        .boxed()
}

fn build_account_by_id_filter(
    handler: &SharedDatabaseHandler,
    templator: Arc<Mutex<Templator>>,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::get()
        .and(warp::path!("id" / i32))
        .and(warp::path::end())
        .map(
            move |id| match handler_clone.lock().unwrap().find_account_by_id(&id) {
                Some(account) => {
                    let pa = PathAndObject::new_account(&account.name, account.id);
                    // Somehow have these warp::reply::html wrap once
                    warp::reply::html(pa.render_file(templator.clone()))
                }
                None => warp::reply::html(format!("Could not find user with id {}", id)),
            },
        )
        .boxed()
}

fn build_account_by_name_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::get()
        .and(warp::path!("name" / String))
        .and(warp::path::end())
        .map(
            move |name| match handler_clone.lock().unwrap().find_account_by_name(&name) {
                Some(account) => format!("{:?}", account),
                None => format!("Could not find user with name {}", name),
            },
        )
        .boxed()
}

fn build_create_account_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::post()
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .map(move |json_map: HashMap<String, String>| {
            if let (Some(username), Some(password)) =
                (json_map.get("username"), json_map.get("password"))
            {
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
        })
        .boxed()
}

fn build_update_account_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::post()
        .and(warp::path("update"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .map(move |json_map: HashMap<String, String>| {
            if let (Some(id), Some(name), Some(password)) = (
                json_map.get("id"),
                json_map.get("name"),
                json_map.get("password"),
            ) {
                if let Ok(id) = i32::from_str_radix(id, 10) {
                    let updated_account = data::account::Account {
                        id,
                        name: name.clone(),
                        password: password.clone(),
                    };

                    match handler_clone
                        .lock()
                        .unwrap()
                        .update_account(&updated_account)
                    {
                        Ok(account) => return format!("Updated account to {:?}", account),
                        Err(error) => {
                            return format!("Failed to update account with error: {}", error)
                        }
                    }
                }
            }
            format!("Failed to update account, name, password, not specified or id not an int.")
        })
        .boxed()
}

fn build_delete_account_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::get()
        .and(warp::path!("id" / i32 / "delete"))
        .and(warp::path::end())
        .map(
            move |id| match handler_clone.lock().unwrap().delete_account(&id) {
                Ok(account) => format!("{:?}", account),
                Err(error) => format!("Failed to delete account id {} with error: {}", id, error),
            },
        )
        .boxed()
}

fn build_tables_by_account_id_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::get()
        .and(warp::path!("id" / i32 / "tables"))
        .and(warp::path::end())
        .map(move |account_id| {
            let tables = handler_clone
                .lock()
                .unwrap()
                .list_tables_by_creator_id(&account_id);
            format!("{:?}", tables)
        })
        .boxed()
}
