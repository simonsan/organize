use crate::configuration::options::combine_options;
use serde::Deserialize;
use std::{
    borrow::Borrow,
    ops::Add,
    path::PathBuf,
};

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ConflictingFileOperation {
    pub to: PathBuf,
    pub if_exists: Option<ConflictOption>,
    pub counter_separator: Option<String>,
}

impl Default for ConflictingFileOperation {
    fn default() -> Self {
        ConflictingFileOperation {
            to: PathBuf::new(), // shouldn't get to this if 'to' isn't specified
            if_exists: Some(Default::default()),
            counter_separator: Some(" ".to_string()),
        }
    }
}

impl Add for ConflictingFileOperation {
    type Output = Self;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The right-hand object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        ConflictingFileOperation {
            if_exists: combine_options(self.if_exists, rhs.if_exists, Some(Default::default())),
            to: rhs.to,
            counter_separator: combine_options(self.counter_separator, rhs.counter_separator, Some(Default::default())),
        }
    }
}

impl Add for &ConflictingFileOperation {
    type Output = ConflictingFileOperation;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The right-hand object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        ConflictingFileOperation {
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
    ask,
}

impl Default for ConflictOption {
    fn default() -> Self {
        ConflictOption::rename
    }
}
