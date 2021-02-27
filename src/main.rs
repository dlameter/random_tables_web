use diesel::prelude::*;
use serde::Deserialize;
use serde_json;
use warp::{http::Response, Filter, Rejection};

use random_tables_web;
use random_tables_web::data;
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
    let edit_name = warp::put()
        .and(warp::path!("change-username"))
        .and(warp::path::end())
        .and(session_closure())
        .and(warp::body::json())
        .and_then(|session, data: ChangeUsernameData| async move {
            change_username(&session, &data.username)
        });
    let edit_pass = warp::put()
        .and(warp::path!("change-password"))
        .and(warp::path::end())
        .and(session_closure())
        .and(warp::body::json())
        .and_then(|session, data: ChangePasswordData| async move {
            change_username(&session, &data.password)
        });

    let auth = login
        .or(logout)
        .or(signup)
        .or(whois)
        .or(edit_name)
        .or(edit_pass);

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
    let cookie = session.authenticate(&login_data.username, &login_data.password);
    let body = session
        .account()
        .as_ref()
        .ok_or("Account not found".to_string())
        .and_then(|account| {
            serde_json::to_string(&account)
                .map_err(|error| format!("Failed to create json string for account: {}", error))
        });

    match body {
        Ok(body) => Response::builder()
            .header("set-cookie", format!("EXAUTH={}", cookie.unwrap()))
            .status(warp::http::StatusCode::OK)
            .body(body)
            .map_err(|error| warp::reject::reject()),
        Err(error) => {
            println!("Failed to find account: {}", error);
            Response::builder()
                .status(warp::http::StatusCode::NOT_FOUND)
                .body("".to_string())
                .map_err(|error| warp::reject::reject())
        }
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
    if let Some(account) = session.logged_in() {
        if let Ok(string) = serde_json::to_string(account) {
            return Response::builder()
                .status(warp::http::StatusCode::OK)
                .body(string)
                .map_err(|error| warp::reject());
        }
    }
    Response::builder()
        .status(warp::http::StatusCode::UNAUTHORIZED)
        .body(String::new())
        .map_err(|error| warp::reject())
}

#[derive(Deserialize)]
struct ChangeUsernameData {
    username: String,
}

fn change_username(
    session: &session::Session,
    new_username: &String,
) -> Result<Response<String>, Rejection> {
    if let Some(account) = session.logged_in() {
        use random_tables_web::schema::accounts::dsl::*;

        let update = diesel::update(accounts).filter(id.eq(account.id));

        let result = update
            .set(username.eq(new_username))
            .returning(username)
            .get_result::<String>(session.connection())
            .map_err(|error| {
                format!(
                    "Failed to update username for account with id={}: {}",
                    account.id, error
                )
            })
            .and_then(|update_username| {
                let updated_account = data::account::Account {
                    id: account.id,
                    username: update_username.clone(),
                };

                serde_json::to_string(&updated_account)
                    .map_err(|error| format!("Failed to create JSON string of account: {}", error))
            });

        let response = match result {
            Ok(response_string) => Response::builder()
                .status(warp::http::StatusCode::OK)
                .body(response_string),
            Err(error_string) => Response::builder()
                .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(error_string),
        };

        return response.map_err(|error| warp::reject());
    }
    Response::builder()
        .status(warp::http::StatusCode::UNAUTHORIZED)
        .body("No user logged in".to_string())
        .map_err(|error| warp::reject())
}

#[derive(Deserialize)]
struct ChangePasswordData {
    password: String,
}

fn change_password(
    session: &session::Session,
    new_password: &String,
) -> Result<Response<String>, Rejection> {
    if let Some(account) = session.logged_in() {
        use random_tables_web::schema::accounts::dsl::*;

        let update = diesel::update(accounts).filter(id.eq(account.id));
        let new_password_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
            .map_err(|error| format!("Failed to hash password: {}", error));

        let result = new_password_hash.and_then(|new_password_hash| {
            update
                .set(password_hash.eq(new_password_hash))
                .execute(session.connection())
                .map(|rows| (rows > 0).to_string())
                .map_err(|error| {
                    format!(
                        "Failed to update password for account with id={}: {}",
                        account.id, error
                    )
                })
        });

        let response = match result {
            Ok(response_string) => Response::builder()
                .status(warp::http::StatusCode::OK)
                .body(response_string),
            Err(error_string) => Response::builder()
                .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(error_string),
        };

        return response.map_err(|error| warp::reject());
    }
    Response::builder()
        .status(warp::http::StatusCode::UNAUTHORIZED)
        .body("No user logged in".to_string())
        .map_err(|error| warp::reject())
}
