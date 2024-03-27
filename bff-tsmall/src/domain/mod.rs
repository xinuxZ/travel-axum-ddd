// mod domain;

pub mod articles;
pub mod comments;
pub mod profiles;
pub mod suppliers;
pub mod tags;
pub mod users;
pub mod utils;

pub use articles::*;
pub use comments::*;
pub use profiles::*;
pub use suppliers::*;
pub use tags::*;
pub use users::*;
pub use utils::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub errors: HashMap<String, Vec<String>>,
}

impl ApiError {
    pub fn new(error: String) -> Self {
        let mut error_map: HashMap<String, Vec<String>> = HashMap::new();
        error_map.insert("message".to_owned(), vec![error]);
        Self { errors: error_map }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct PingResponse {
    pub message: String,
}

impl PingResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Default for PingResponse {
    fn default() -> Self {
        Self::new(String::from("API is responsive"))
    }
}
