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
    let update_account = warp::put()
        .and(warp::path!("account" / i32))
        .and(warp::path::end())
        .and(session_closure())
        .and(warp::body::json())
        .and_then(|id, session, edit_data| async move { edit_account(session, id, edit_data) });

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

#[derive(Deserialize)]
struct EditAccountData {
    username: Option<String>,
    password: Option<String>,
}

impl EditAccountData {
    fn is_empty(&self) -> bool {
        self.username.is_none() && self.password.is_none()
    }
}

fn edit_account(
    session: session::Session,
    account_id: i32,
    edit_data: EditAccountData,
) -> Result<Response<String>, Rejection> {
    if session.logged_in() {
        if let Some(session_account) = session.account() {
            if !edit_data.is_empty() {
                if session_account.id == account_id {
                    let mut errors: Vec<String> = vec![];
                    use random_tables_web::schema::accounts::dsl::*;
                    let update = diesel::update(accounts).filter(id.eq(account_id));

                    let mut result_username = None;
                    if let Some(new_username) = edit_data.username {
                        result_username = Some(
                            update
                                .set(username.eq(new_username))
                                .returning(username)
                                .get_result::<String>(session.connection()),
                        );
                    }

                    let mut result_password = None; // Option<Result<String, diesel::Error>>
                    if let Some(new_password) = edit_data.password {
                        let new_password = bcrypt::hash(&new_password, bcrypt::DEFAULT_COST);

                        match new_password {
                            Ok(new_password) => {
                                result_password = Some(
                                    update
                                        .set(password_hash.eq(new_password))
                                        .returning(password_hash)
                                        .get_result::<String>(session.connection()),
                                );
                            }
                            Err(error) => errors.push(format!("{}", error).to_string()),
                        }
                    }

                    let mut result_account = data::account::Account {
                        id: account_id,
                        username: session_account.username.clone(),
                    };

                    result_username.and_then(|result| {
                        let err = result.and_then(|new_username| {
                            result_account.username = new_username;
                            Ok(())
                        });
                        if let Err(err) = err {
                            let error = format!("{}", err).to_string();
                            errors.push(error.clone());
                            return Some(error);
                        }
                        None
                    });

                    result_password.and_then(|result| {
                        let err = result.and_then(|new_password| Ok(()));
                        if let Err(err) = err {
                            let error = format!("{}", err).to_string();
                            errors.push(error.clone());
                            return Some(error);
                        }
                        None
                    });

                    if errors.is_empty() {
                        return Response::builder()
                            .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                            .body(
                                serde_json::to_string(&errors)
                                    .unwrap_or("Failed to JSON-ify errors.".to_string()),
                            )
                            .map_err(|error| warp::reject());
                    } else {
                        return Response::builder()
                            .status(warp::http::StatusCode::OK)
                            .body(
                                serde_json::to_string(&result_account).unwrap_or(
                                    "Failed to create JSON for updated data.".to_string(),
                                ),
                            )
                            .map_err(|error| warp::reject());
                    }
                }
            }
        }
    }
    Response::builder()
        .status(warp::http::StatusCode::UNAUTHORIZED)
        .body(String::new())
        .map_err(|error| warp::reject())
}
