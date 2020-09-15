use serde::Deserialize;
use std::path::PathBuf;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ConflictingFileOperation {
    pub to: Option<PathBuf>,
    pub if_exists: Option<ConflictOption>,
    pub counter_separator: Option<String>,
}

impl Default for ConflictingFileOperation {
    fn default() -> Self {
        ConflictingFileOperation {
            to: None,
            if_exists: Some(Default::default()),
            counter_separator: Some(" ".to_string()),
        }
    }
}

/// Defines the options available to resolve a naming conflict,
/// i.e. how the application should proceed when a file exists
/// but it should move/rename/copy some file to that existing path
#[allow(non_camel_case_types)]
// if set with camelCase or PascalCase the user would have to
// write their configs with this format due to how serde deserializes files
// and so it would be inconsistent with the rest of the config file
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub enum ConflictOption {
    overwrite,
    skip,
    rename,
    ask,
}

impl Default for ConflictOption {
    fn default() -> Self {
        ConflictOption::rename
    }
}
