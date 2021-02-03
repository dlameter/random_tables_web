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

    let routes = cookie_test.with(cors.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
