use serde::{
    Deserialize,
    Serialize,
};
use std::path::PathBuf;

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

// TODO revise this implementation after the defaults' section is implemented
impl Default for Options {
    fn default() -> Self {
        Self {
            recursive: false,
            watch: false,
            hidden_files: false,
            suggestions: false,
            ignore: Vec::new(),
        }
    }
}
