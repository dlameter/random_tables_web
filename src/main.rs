use warp::{filters::BoxedFilter, Filter, Reply};

use random_tables_web::data;
use random_tables_web::session::pg_pool;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type", "cookie"])
        .allow_credentials(true);

    let cookie: BoxedFilter<(String,)> = warp::any()
        .and(warp::filters::cookie::optional("EXAUTH"))
        .and_then(move |key: Option<String>| async move {
            match key {
                Some(value) => Ok::<String, std::convert::Infallible>(value),
                None => Ok("new_cookie".to_string()),
            }
        })
        .boxed();

    let cookie_test = warp::get()
        .and(warp::path!("cookie"))
        .and(warp::path::end())
        .and(cookie)
        .map(|key| {
            warp::reply::with_header(warp::reply(), "Set-Cookie", format!("EXAUTH={}", key))
        });

    let routes = cookie_test.with(cors.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
