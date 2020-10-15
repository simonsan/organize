use super::deserialize::{
    default_regex,
    deserialize_regex,
};
use regex::Regex;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Filters {
    #[serde(deserialize_with = "deserialize_regex", default = "default_regex")]
    pub regex: Regex,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub extensions: Vec<String>,
}

#[allow(clippy::trivial_regex)]
impl Default for Filters {
    fn default() -> Self {
        Filters {
            regex: Regex::new("").unwrap(),
            filename: String::new(),
            extensions: Vec::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
struct Filename {
    #[serde(default)]
    startswith: String,
    #[serde(default)]
    endswith: String,
    #[serde(default)]
    contains: String,
    #[serde(default)]
    case_sensitive: bool,
}
