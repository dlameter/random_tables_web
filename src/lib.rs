use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use liquid::object;
use liquid::*;
use warp::Filter;
use warp::{filters::BoxedFilter, http::Uri};

mod data;
mod database_handler;
mod templating;

use templating::Templator;

pub struct PathAndObject {
    path: PathBuf,
    object: Object,
}

impl PathAndObject {
    pub fn new_account(name: &str, id: i32) -> Self {
        Self {
            path: PathBuf::from("account.html"),
            object: object!({
            "username": name.to_string(),
            "userid": id,
            }),
        }
    }

    pub fn render_file(self, templator: Arc<Mutex<Templator>>) -> String {
        let guard = templator.lock().unwrap();
        guard.render_file(&self.path, &self.object)
    }
}
