use super::deserialize::deserialize_path;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Folder {
    #[serde(deserialize_with = "deserialize_path")]
    pub path: PathBuf,
    #[serde(default)]
    pub options: Options,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Options {
    /// defines whether or not subdirectories must be scanned
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub watch: bool,
    #[serde(default)]
    pub ignore: Vec<PathBuf>,
    #[serde(default)]
    pub suggestions: bool,
    #[serde(default)]
    pub hidden_files: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            recursive: false,
            watch: true,
            hidden_files: false,
            suggestions: false,
            ignore: Vec::new(),
        }
    }
}
