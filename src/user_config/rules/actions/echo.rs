use crate::string::Placeholder;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Echo(String);

impl Echo {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn run(&self, path: &Path) {
        println!("{}", self.0.expand_placeholders(path).unwrap());
    }
}
