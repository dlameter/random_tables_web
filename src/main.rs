use warp::{http::Uri, Filter};

#[tokio::main]
async fn main() {
    let index = warp::path("index.html").and(warp::fs::file("index.html"));
    let index_redirect = warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html")));

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(index.or(index_redirect).or(hello));

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}
