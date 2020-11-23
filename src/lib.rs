use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use liquid::object;

pub mod data;
pub mod database_handler;
pub mod templating;

use templating::Templator;

pub struct PathAndObject {
    path: PathBuf,
    object: liquid::Object,
}

impl PathAndObject {
    pub fn default() -> Self {
        Self {
            path: PathBuf::from("index.html"),
            object: object!({}),
        }
    }

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
