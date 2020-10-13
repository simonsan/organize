use regex::Regex;
use serde::{Deserialize, Serialize, Deserializer};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Filters {
    pub regex: String,
    pub filename: String,
    pub extensions: Vec<String>,
}

impl Default for Filters {
    fn default() -> Self {
        Filters {
            regex: String::new(),
            filename: String::new(),
            extensions: Vec::new(),
        }
    }
}
