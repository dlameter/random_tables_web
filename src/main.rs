use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path("index.html").or(warp::path::end()).map(|_| "This is the index page!");

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(index.or(hello));

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}
