use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
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


