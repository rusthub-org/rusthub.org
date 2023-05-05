pub mod home;
pub mod users;
pub mod creations;
pub mod topics;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Page {
    pub from: i64,
    pub first: String,
    pub last: String,
}

impl Default for Page {
    fn default() -> Self {
        Self { from: 1, first: String::from("-"), last: String::from("-") }
    }
}
