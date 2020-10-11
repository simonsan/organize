use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct TemporaryFilters {
    pub regex: Option<String>,
    pub filename: Option<String>,
    pub extensions: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Filters {
    pub regex: Regex,
    pub filename: String,
    pub extensions: Vec<String>,
}

impl Default for TemporaryFilters {
    fn default() -> Self {
        TemporaryFilters {
            regex: Some(String::new()),
            filename: Some(String::new()),
            extensions: Some(Vec::new()),
        }
    }
}

impl TemporaryFilters {
    pub fn unwrap(&self) -> Filters {
        Filters {
            regex: Regex::from_str(self.regex.clone().unwrap_or_default().as_str()).expect("invalid regex"),
            filename: self.filename.clone().unwrap_or_default(),
            extensions: self.extensions.clone().unwrap_or_default(),
        }
    }
}
