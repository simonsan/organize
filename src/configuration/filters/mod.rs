use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
mod lib;

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
