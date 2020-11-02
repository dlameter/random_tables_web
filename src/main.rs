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

    let account_by_id = warp::path!("account" / i32)
        .map(move |id| {
            match handler.clone()
                .lock()
                .unwrap()
                .find_account_by_id(&id) {
                    Some(account) => format!("{:?}", account),
                    None => format!("Could not find user with id {}", id),
            }
        });

    let templator = Templator::new("./_layout/".to_string(), "./_includes/".to_string());
    let templator = Arc::new(templator);
    
    let template_file = move |file_path| warp::reply::html(templator.clone().render_file(file_path, &object!({})));

    let index = warp::path("index.html")
        .map(|| Path::new("index.html"))
        .map(template_file);
    let index_redirect = warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html")));

    let static_files = warp::path("static").and(warp::fs::dir("static"));

    let routes = warp::get().and(index.or(index_redirect).or(static_files).or(account_by_id));

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}
