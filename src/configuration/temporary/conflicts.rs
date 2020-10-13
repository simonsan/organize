use serde::{
    Deserialize,
    Serialize,
};
use std::path::PathBuf;

#[derive(Clone)]
pub enum ConflictingActions {
    Move,
    Rename,
    Delete,
    None,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ConflictingFileOperation {
    pub to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub counter_separator: String,
}

impl Default for ConflictingFileOperation {
    fn default() -> Self {
        Self {
            to: PathBuf::new(), // shouldn't get to this if 'to' isn't specified
            if_exists: Default::default(),
            counter_separator: " ".to_string(),
        }
    }
}

/// Defines the options available to resolve a naming conflict,
/// i.e. how the application should proceed when a file exists
/// but it should move/rename/copy some file to that existing path
// write their configs with this format due to how serde deserializes files
#[derive(Eq, PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum ConflictOption {
    Overwrite,
    Skip,
    Rename,
    Ask, // not available when watching
}

impl Default for ConflictOption {
    fn default() -> Self {
        ConflictOption::Rename
    }
}

impl Default for &ConflictOption {
    fn default() -> Self {
        &ConflictOption::Rename
    }
}
