use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use warp::{http::Response, Filter, Rejection};

use random_tables_web;
use random_tables_web::session;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
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
    let signup = warp::post()
        .and(warp::path("signup"))
        .and(warp::path::end())
        .and(session_closure())
        .and(warp::body::json())
        .and_then(|session, signup_data| async move { do_signup(session, signup_data) });
    let whois = warp::get()
        .and(warp::path("whois"))
        .and(warp::path::end())
        .and(session_closure())
        .and_then(|session| async move { do_whois(session) });

    let auth = login.or(logout).or(signup).or(whois);

    let routes = auth.with(cors);

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
            .status(warp::http::StatusCode::OK)
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
        .status(warp::http::StatusCode::OK)
        .body(String::new())
        .map_err(|error| warp::reject())
}

type SignupData = LoginData;

impl SignupData {
    fn validate(self) -> Result<Self, &'static str> {
        if self.username.len() < 3 {
            Err("Username must be at least 3 characters")
        } else if self.password.len() < 8 {
            Err("Password must be at least 8 characters")
        } else {
            Ok(self)
        }
    }
}

fn do_signup(
    session: session::Session,
    signup_data: SignupData,
) -> Result<Response<String>, Rejection> {
    let result = signup_data
        .validate()
        .map_err(|error| error.to_string())
        .and_then(|signup_data| {
            let hash = bcrypt::hash(&signup_data.password, bcrypt::DEFAULT_COST)
                .map_err(|error| format!("Failed to hash password: {}", error))?;
            Ok((hash, signup_data))
        })
        .and_then(|(hash, signup_data)| {
            use random_tables_web::schema::accounts::dsl::*;
            diesel::insert_into(accounts)
                .values((username.eq(&signup_data.username), password_hash.eq(&hash)))
                .execute(session.connection())
                .map_err(|error| format!("{:?}", error).to_string())
        });
    match result {
        Ok(_) => Response::builder()
            .status(warp::http::StatusCode::CREATED)
            .body(String::new())
            .map_err(|error| warp::reject()),
        Err(error) => Response::builder()
            .status(warp::http::StatusCode::BAD_REQUEST)
            .body(format!("Failed to create account: {}", error))
            .map_err(|error| warp::reject()),
    }
}

fn do_whois(session: session::Session) -> Result<Response<String>, Rejection> {
    if session.logged_in() {
        if let Some(account) = session.account() {
            if let Ok(string) = serde_json::to_string(account) {
                return Response::builder()
                    .status(warp::http::StatusCode::OK)
                    .body(string)
                    .map_err(|error| warp::reject());
            }
        }
    }
    Response::builder()
        .status(warp::http::StatusCode::UNAUTHORIZED)
        .body(String::new())
        .map_err(|error| warp::reject())
}
