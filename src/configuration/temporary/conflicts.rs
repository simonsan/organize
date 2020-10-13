use serde::{
    Deserialize,
    Serialize
};
use std::{
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
#[serde(default)]
pub struct ConflictingFileOperationOptions {
    pub if_exists: ConflictOption,
    pub counter_separator: String,
}

impl Default for ConflictingFileOperationOptions {
    fn default() -> Self {
        Self {
            if_exists: Default::default(),
            counter_separator: " ".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ConflictingFileOperation {
    // #[serde(deserialize_with = "expand_env_vars")]
    pub to: PathBuf,
    pub options: ConflictingFileOperationOptions,
}

// impl ConflictingFileOperation {
//     pub fn unwrap(self) -> ConflictingFileOperation {
//         ConflictingFileOperation {
//             counter_separator: self.counter_separator.unwrap(),
//             if_exists: self.if_exists.unwrap(),
//             to: self.to,
//         }
//     }
//
//     pub fn fill(&mut self) {
//         self.to = expand_env_vars(&self.to);
//     }
//
//     pub fn is_complete(&self) -> bool {
//         self.if_exists.is_some() && self.counter_separator.is_some()
//     }
// }

impl Default for ConflictingFileOperation {
    fn default() -> Self {
        Self {
            to: PathBuf::new(), // shouldn't get to this if 'to' isn't specified
            options: Default::default(),
        }
    }
}
//
// impl Add for ConflictingFileOperation {
//     type Output = Self;
//
//     /// Performs the + operation.
//     /// This addition is not commutative.
//     /// The right-hand object's fields are prioritized.
//     fn add(self, rhs: Self) -> Self::Output {
//         ConflictingFileOperation {
//             if_exists: combine_options(self.if_exists, rhs.if_exists, Some(Default::default())),
//             to: rhs.to,
//             counter_separator: combine_options(self.counter_separator, rhs.counter_separator, Some(Default::default())),
//         }
//     }
// }
//
// impl Add for &ConflictingFileOperation {
//     type Output = ConflictingFileOperation;
//
//     /// Performs the + operation.
//     /// This addition is not commutative.
//     /// The right-hand object's fields are prioritized.
//     fn add(self, rhs: Self) -> Self::Output {
//         ConflictingFileOperation {
//             if_exists: combine_options(self.clone().if_exists, rhs.clone().if_exists, Some(Default::default())),
//             to: rhs.clone().to,
//             counter_separator: combine_options(
//                 self.clone().counter_separator,
//                 rhs.clone().counter_separator,
//                 Some(Default::default()),
//             ),
//         }
//     }
// }

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
