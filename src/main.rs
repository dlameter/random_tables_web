use serde::Deserialize;
use warp::{filters::BoxedFilter, http::Response, Filter, Rejection, Reply};

use random_tables_web;
use random_tables_web::{data, session};

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type", "cookie"])
        .allow_credentials(true);
    let sessions = session::create_optional_session_filter(&random_tables_web::get_database_url());
    let session_closure = move || sessions.clone();

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(session_closure())
        .and(warp::body::json())
        .and_then(|session, login_data| async move { do_login(session, login_data) });
    let logout = warp::post()
        .and(warp::path("logout"))
        .and(warp::path::end())
        .and(session_closure())
        .and_then(|session| async move { do_logout(session) });

    let auth = login.or(logout);

    let routes = auth.with(cors.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

fn do_login(
    mut session: session::Session,
    login_data: LoginData,
) -> Result<Response<String>, Rejection> {
    if let Some(cookie) = session.authenticate(&login_data.username, &login_data.password) {
        Response::builder()
            .header("set-cookie", format!("EXAUTH={}", cookie))
            .status(warp::http::StatusCode::FOUND)
            .body("".to_string())
            .map_err(|error| warp::reject::reject())
    } else {
        Response::builder()
            .status(warp::http::StatusCode::NOT_FOUND)
            .body("".to_string())
            .map_err(|error| warp::reject::reject())
    }
}

fn do_logout(mut session: session::Session) -> Result<Response<String>, Rejection> {
    session.clear();
    Response::builder()
        .status(warp::http::StatusCode::FOUND)
        .body(String::new())
        .map_err(|error| warp::reject())
}
