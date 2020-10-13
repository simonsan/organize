use serde::{
    Deserialize,
    Serialize,
};

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

struct Filename {
    startswith: Option<String>,
    endswith: Option<String>,
    contains: Option<String>,
    case_sensitive: Option<bool>
}
