use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Actions {
    echo: Option<String>,
    shell: Option<String>,
    trash: Option<bool>,
    delete: Option<bool>,
    copy: Option<ConflictingFileOperation>,
    r#move: Option<ConflictingFileOperation>,
    rename: Option<ConflictingFileOperation>,
}

impl Default for Actions {
    fn default() -> Self {
        Actions {
            echo: None,
            shell: None,
            trash: Some(false),
            delete: Some(false),
            copy: Some(Default::default()),
            r#move: Some(Default::default()),
            rename: Some(Default::default()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ConflictingFileOperation {
    to: Option<String>,
    enabled: Option<bool>,
    if_exists: Option<ConflictOption>,
    counter_separator: Option<String>,
}

impl Default for ConflictingFileOperation {
    fn default() -> Self {
        ConflictingFileOperation {
            to: None,
            enabled: Some(false),
            if_exists: Some(Default::default()),
            counter_separator: Some(" ".to_string()),
        }
    }
}

impl ConflictingFileOperation {
    pub fn try_enable(&mut self) {
        if self.to.is_some() && self.to.as_ref().unwrap().ne("") {
            self.enabled = Some(true)
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
