use super::deserialize::{default_regex, deserialize_regex};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Filters {
    #[serde(deserialize_with = "deserialize_regex", default = "default_regex")]
    pub regex: Regex,
    #[serde(default)]
    pub filename: Filename,
    #[serde(default)]
    pub extensions: Vec<String>,
}

impl PartialEq for Filters {
    fn eq(&self, other: &Self) -> bool {
        self.regex.to_string() == other.regex.to_string()
            && self.filename == other.filename
            && self.extensions == other.extensions
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[allow(clippy::trivial_regex)]
impl Default for Filters {
    fn default() -> Self {
        Filters {
            regex: default_regex(),
            filename: Default::default(),
            extensions: Vec::new(),
        }
    }
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug, Clone, Default)]
pub struct Filename {
    #[serde(default)]
    pub startswith: String,
    #[serde(default)]
    pub endswith: String,
    #[serde(default)]
    pub contains: String,
    #[serde(default)]
    pub case_sensitive: bool,
}
