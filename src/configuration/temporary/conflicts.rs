use crate::configuration::{
    combine_options,
    temporary::folders::expand_env_vars,
};
use serde::Deserialize;
use std::{
    ops::Add,
    path::PathBuf,
};

#[derive(Clone)]
pub enum ConflictingActions {
    Move,
    Rename,
    Delete,
    None,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct TemporaryConflictingFileOperation {
    pub to: PathBuf,
    pub if_exists: Option<ConflictOption>,
    pub counter_separator: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConflictingFileOperation {
    pub to: PathBuf,
    pub if_exists: ConflictOption,
    pub counter_separator: String,
}

impl TemporaryConflictingFileOperation {
    pub fn unwrap(self) -> ConflictingFileOperation {
        ConflictingFileOperation {
            counter_separator: self.counter_separator.unwrap(),
            if_exists: self.if_exists.unwrap(),
            to: self.to,
        }
    }

    pub fn fill(&mut self) {
        self.if_exists = Some(self.if_exists.clone().unwrap_or_default());
        self.counter_separator = Some(self.counter_separator.clone().unwrap_or_else(|| " ".to_string()));
        self.to = expand_env_vars(&self.to);
    }

    pub fn is_complete(&self) -> bool {
        self.if_exists.is_some() && self.counter_separator.is_some()
    }
}

impl Default for TemporaryConflictingFileOperation {
    fn default() -> Self {
        TemporaryConflictingFileOperation {
            to: PathBuf::new(), // shouldn't get to this if 'to' isn't specified
            if_exists: Some(Default::default()),
            counter_separator: Some(" ".to_string()),
        }
    }
}

impl Add for TemporaryConflictingFileOperation {
    type Output = Self;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The right-hand object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        TemporaryConflictingFileOperation {
            if_exists: combine_options(self.if_exists, rhs.if_exists, Some(Default::default())),
            to: rhs.to,
            counter_separator: combine_options(self.counter_separator, rhs.counter_separator, Some(Default::default())),
        }
    }
}

impl Add for &TemporaryConflictingFileOperation {
    type Output = TemporaryConflictingFileOperation;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The right-hand object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        TemporaryConflictingFileOperation {
            if_exists: combine_options(self.clone().if_exists, rhs.clone().if_exists, Some(Default::default())),
            to: rhs.clone().to,
            counter_separator: combine_options(
                self.clone().counter_separator,
                rhs.clone().counter_separator,
                Some(Default::default()),
            ),
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
    ask, // not available when watching
}

impl Default for ConflictOption {
    fn default() -> Self {
        ConflictOption::rename
    }
}

impl Default for &ConflictOption {
    fn default() -> Self {
        &ConflictOption::rename
    }
}
