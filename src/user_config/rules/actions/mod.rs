use copy::Copy;
use r#move::Move;
use std::{io::Result, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::deserialize::deserialize_path;
use crate::user_config::rules::actions::{delete::Delete, echo::Echo, rename::Rename, shell::Shell};

pub mod copy;
pub mod delete;
pub mod echo;
pub mod r#move;
pub mod rename;
pub mod shell;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Sep(String);

impl Default for Sep {
    fn default() -> Self {
        Self(" ".into())
    }
}

impl Sep {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub enum Action {
    Move,
    Rename,
    Copy,
    Delete,
    Trash,
    Echo,
    Shell,
}

impl From<&str> for Action {
    fn from(str: &str) -> Self {
        match str.to_lowercase().as_str() {
            "move" => Self::Move,
            "copy" => Self::Copy,
            "rename" => Self::Rename,
            "delete" => Self::Delete,
            "trash" => Self::Trash,
            "echo" => Self::Echo,
            "shell" => Self::Shell,
            _ => panic!("unknown action"),
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Self::Move => "move",
            Self::Copy => "copy",
            Self::Rename => "rename",
            Self::Delete => "delete",
            Self::Trash => "trash",
            Self::Echo => "echo",
            Self::Shell => "shell",
        }
        .into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Actions {
    pub echo: Option<Echo>,
    pub shell: Option<Shell>,
    pub delete: Option<Delete>,
    pub copy: Option<Copy>,
    pub r#move: Option<Move>,
    pub rename: Option<Rename>,
}

impl Actions {
    pub fn run(&self, mut path: PathBuf) -> Result<()> {
        assert!((self.r#move.is_some() ^ self.rename.is_some()) || self.r#move.is_none() && self.rename.is_none());
        if let Some(echo) = &self.echo {
            echo.run(&path);
        }
        if let Some(shell) = &self.shell {
            shell.run(&path)?;
        }
        if let Some(copy) = &self.copy {
            copy.run(&path)?;
        }
        if self.r#move.is_some() ^ self.rename.is_some() {
            let mut result = PathBuf::new();
            if let Some(r#move) = &self.r#move {
                if let Some(path) = r#move.run(&path)? {
                    result = path;
                }
            } else if let Some(rename) = &self.rename {
                if let Some(path) = rename.run(&path)? {
                    result = path;
                }
            }
            path = result;
        }
        if let Some(delete) = &self.delete {
            if delete.as_bool() {
                delete.run(&path)?;
            }
        }
        Ok(())
    }
}

/// Defines the options available to resolve a naming conflict,
/// i.e. how the application should proceed when a file exists
/// but it should move/rename/copy some file to that existing path
// write their configs with this format due to how serde deserializes files
#[derive(Eq, PartialEq, Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum ConflictOption {
    Overwrite,
    Skip,
    Rename,
    Delete,
    Ask, // not available when watching
}

impl Default for ConflictOption {
    fn default() -> Self {
        ConflictOption::Rename
    }
}

impl ConflictOption {
    pub fn should_skip(&self) -> bool {
        self == &Self::Skip
    }

    pub fn should_overwrite(&self) -> bool {
        self == &Self::Overwrite
    }

    pub fn should_delete(&self) -> bool {
        self == &Self::Delete
    }

    pub fn should_rename(&self) -> bool {
        self == &Self::Rename
    }

    pub fn should_ask(&self) -> bool {
        self == &Self::Ask
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind, Result};

    use crate::{
        path::{
            lib::vars::{expected_path, test_file_or_dir},
            Update,
        },
        user_config::rules::actions::ConflictOption,
    };

    #[test]
    fn rename_with_rename_conflict() -> Result<()> {
        let target = test_file_or_dir("test2.txt");
        let expected = expected_path(&target, &Default::default());
        let new_path = target.update(&ConflictOption::Rename, &Default::default()).unwrap();
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_overwrite_conflict() -> Result<()> {
        let target = test_file_or_dir("test2.txt");
        let expected = target.clone();
        let new_path = target.update(&ConflictOption::Overwrite, &Default::default()).unwrap();
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    #[should_panic] // unwrapping a None value
    fn rename_with_skip_conflict() {
        let target = test_file_or_dir("test2.txt");
        target.update(&ConflictOption::Skip, &Default::default()).unwrap();
    }

    #[test]
    #[should_panic] // trying to modify a path that does not exist
    fn new_path_to_non_existing_file() {
        let target = test_file_or_dir("test_dir2").join("test1.txt");
        assert!(!target.exists());
        target.update(&ConflictOption::Rename, &Default::default()).unwrap();
    }
}
