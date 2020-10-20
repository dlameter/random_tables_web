use std::path::Path;
use std::sync::Arc;

use warp::{http::Uri, Filter};

mod templating;
use templating::Templator;

#[tokio::main]
async fn main() {
    let templator = Templator::new("./_layout/".to_string(), "./_includes/".to_string());
    let templator = Arc::new(templator);
    
    let template_file = move |file_path| warp::reply::html(templator.clone().render_file(file_path));

    let index = warp::path("index.html").and(warp::fs::file("index.html"));
    let index_redirect = warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html")));

    let test = warp::path("test")
        .map(|| Path::new("index.html"))
        .map(template_file);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let static_files = warp::path("static").and(warp::fs::dir("static"));

    let routes = warp::get().and(index.or(index_redirect).or(hello).or(test).or(static_files));

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}
