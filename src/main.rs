use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use config;
use warp::{Filter, filters::BoxedFilter, Reply};

use random_tables_web::data;
use random_tables_web::database_handler::{DatabaseConfig, DatabaseHandler};

type SharedDatabaseHandler = Arc<Mutex<DatabaseHandler>>;

#[tokio::main]
async fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config.json")).unwrap();
    
    let dbconfig = DatabaseConfig::new(
        settings.get_str("host").unwrap(),
        settings.get_str("dbname").unwrap(),
        settings.get_str("user").unwrap(),
        settings.get_str("password").unwrap()
    );

    let handler = match DatabaseHandler::new(&dbconfig) {
        Ok(c) => c,
        Err(e) => panic!(format!(
            "Failed to create database handler with error: {}",
            e
        )),
    };
    let handler = Arc::new(Mutex::new(handler));

    let accounts_endpoint = build_account_endpoint(&handler);

    let cors = warp::cors().allow_origin("http://localhost:3000");

    let routes = accounts_endpoint.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn build_account_endpoint(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path("account")
        .and(
            build_account_by_id_filter(handler)
                .or(build_account_by_name_filter(handler))
                .or(build_tables_by_account_id_filter(handler))
                .or(build_delete_account_filter(handler))
                .or(build_create_account_filter(handler))
                .or(build_update_account_filter(handler))
        )
        .boxed()
}

fn build_account_by_id_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::get()
        .and(warp::path!("id" / i32))
        .and(warp::path::end())
        .map(
            move |id| match handler_clone.lock().unwrap().find_account_by_id(&id) {
                Some(account) => warp::reply::with_status(
                    warp::reply::json(&account), 
                    warp::http::StatusCode::OK
                ).into_response(),
                None => warp::reply::with_status(
                    warp::reply(), 
                    warp::http::StatusCode::NOT_FOUND
                ).into_response(),
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
                Some(account) => warp::reply::with_status(
                    warp::reply::json(&account), 
                    warp::http::StatusCode::OK
                ).into_response(),
                None => warp::reply::with_status(
                    warp::reply(), 
                    warp::http::StatusCode::NOT_FOUND
                ).into_response(),
            },
        )
        .boxed()
}

fn build_create_account_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::post()
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .map(move |account: data::account::Account| {
            match handler_clone.lock().unwrap().create_account(&account) {
                Ok(created_account) => warp::reply::with_status(
                    format!("Account created with id {}", created_account.id.unwrap()),
                    warp::http::StatusCode::CREATED
                ),
                Err(error) => warp::reply::with_status(
                    format!("Failed to create account with error: {}", error),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR
                ),
            }
        })
        .boxed()
}

fn build_update_account_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::put()
        .and(warp::path!("id" / i32))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .map(move |id, account: data::account::Account| {
            let updated_account = data::account::Account {
                id: Some(id),
                name: account.name.clone(),
                password: account.password.clone(),
            };

            match handler_clone
                .lock()
                .unwrap()
                .update_account(&updated_account)
            {
                Ok(account) => warp::reply::with_status(
                    format!("Updated account with id {}", account.id.unwrap()),
                    warp::http::StatusCode::OK
                ),
                Err(error) => warp::reply::with_status(
                    format!("Failed to update account with error: {}", error),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR
                )
            }
        })
        .boxed()
}

fn build_delete_account_filter(
    handler: &SharedDatabaseHandler,
) -> BoxedFilter<(impl warp::Reply,)> {
    let handler_clone = Arc::clone(handler);
    warp::delete()
        .and(warp::path!("id" / i32))
        .and(warp::path::end())
        .map(
            move |id| match handler_clone.lock().unwrap().delete_account(&id) {
                Ok(account) => warp::reply::with_status(
                    warp::reply::json(&account), 
                    warp::http::StatusCode::OK
                ).into_response(),
                Err(error) => warp::reply::with_status(
                    format!("Failed to delete account id {} with error: {}", id, error),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR
                ).into_response(),
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
