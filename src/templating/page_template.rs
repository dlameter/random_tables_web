use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use liquid::object;

use super::templator::Templator;

pub struct PageTemplate {
    path: PathBuf,
    object: liquid::Object,
}

impl PageTemplate {
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

    pub fn edit_account(name: &str, id: i32) -> Self {
        Self {
            path: PathBuf::from("edit_account.html"),
            object: object!({
            "name": name.to_string(),
            "id": id,
            }),
        }
    }

    pub fn render_with(self, templator: Arc<Mutex<Templator>>) -> String {
        let guard = templator.lock().unwrap();
        guard.render_file(&self.path, &self.object)
    }
}
