use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    auth: Option<String>,
}

impl Config {
    // TODO
}
