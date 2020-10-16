mod lib;

use std::{
    fs,
    io::Error,
    path::{
        Path,
        PathBuf,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

use super::deserialize::deserialize_path;
use crate::{
    commands::run::resolve_conflict,
    file::{
        get_stem_and_extension,
        File,
    },
    path::Expandable,
};
use std::io::ErrorKind;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Actions {
    pub echo: Option<String>,
    pub shell: Option<String>,
    pub trash: Option<bool>,
    pub delete: Option<bool>,
    pub copy: Option<ConflictingFileOperation>,
    pub r#move: Option<ConflictingFileOperation>,
    pub rename: Option<ConflictingFileOperation>,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            echo: None,
            shell: None,
            trash: None,
            delete: None,
            copy: None,
            rename: None,
            r#move: None,
        }
    }
}

impl Actions {
    pub fn run(&self, file: &mut File, watching: bool) -> Result<(), Error> {
        if self.copy.is_some() {
            if let Err(e) = self.copy(&file.path, watching) {
                if e.kind() != ErrorKind::AlreadyExists {
                    return Err(e);
                }
            }
        }
        // TODO the following two are conflicting operations - validate this
        if self.r#move.is_some() {
            match self.r#move(&file.path, watching) {
                Ok(path) => {
                    file.path = path;
                }
                Err(e) => {
                    if e.kind() != ErrorKind::AlreadyExists {
                        return Err(e);
                    }
                }
            }
        }
        if self.rename.is_some() {
            match self.rename(&file.path, watching) {
                Ok(path) => {
                    file.path = path;
                }
                Err(e) => {
                    if e.kind() != ErrorKind::AlreadyExists {
                        return Err(e);
                    }
                }
            }
        }
        if self.delete.is_some() {
            self.delete(&file.path)?;
        }
        Ok(())
    }

    fn copy(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(self.copy.is_some());
        let copy = self.copy.as_ref().unwrap();
        if !copy.to.exists() {
            fs::create_dir_all(&copy.to)?;
        }
        let to = copy.to.join(from.file_name().unwrap());
        if to.exists() {
            match copy.get_new_path(from, watching) {
                Ok(to) => {
                    std::fs::copy(from, &to)?;
                    Ok(to)
                }
                Err(e) => {
                    // can only happen if `if_exists` is set to skip or
                    // there is a problem when writing the file
                    Err(e)
                }
            }
        } else {
            let to = copy.to.join(to);
            std::fs::copy(from, &to)?;
            Ok(to)
        }
    }

    fn rename(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(self.rename.is_some());
        let rename = self.rename.as_ref().unwrap();
        let parent = rename.to.parent().unwrap();
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
        if rename.to.exists() {
            match rename.get_new_path(from, watching) {
                Ok(to) => {
                    std::fs::rename(from, &to)?;
                    Ok(to)
                }
                Err(e) => {
                    // can only happen if `if_exists` is set to skip or overwrite, or
                    // there is a problem when writing the file
                    Err(e)
                }
            }
        } else {
            std::fs::rename(from, &rename.to)?;
            Ok(rename.to.clone())
        }
    }

    fn r#move(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(self.r#move.is_some());
        let r#move = self.r#move.as_ref().unwrap();
        if !r#move.to.exists() {
            fs::create_dir_all(&r#move.to)?;
        }
        let to = r#move.to.join(from.file_name().unwrap());
        if to.exists() {
            match r#move.get_new_path(from, watching) {
                Ok(to) => {
                    std::fs::rename(from, &to)?;
                    Ok(to)
                }
                Err(e) => {
                    // can only happen if `if_exists` is set to skip or
                    // there is a problem when writing the file
                    Err(e)
                }
            }
        } else {
            std::fs::rename(from, &to)?;
            Ok(to)
        }
    }

    fn delete(&self, path: &Path) -> Result<(), Error> {
        std::fs::remove_file(path)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConflictingFileOperation {
    #[serde(deserialize_with = "deserialize_path")]
    pub(crate) to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub counter_separator: String,
}

impl From<&str> for ConflictingFileOperation {
    fn from(path: &str) -> Self {
        let mut op = ConflictingFileOperation::default();
        op.to = PathBuf::from(path).expand_vars();
        op
    }
}

impl From<PathBuf> for ConflictingFileOperation {
    fn from(path: PathBuf) -> Self {
        let mut op = ConflictingFileOperation::default();
        op.to = path.expand_vars();
        op
    }
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

impl ConflictingFileOperation {
    /// Computes the appropriate new path based on `self.if_exists`.
    /// # Args
    /// * `from`: path representing the original file's path
    /// * `watching`: whether this function is being run from a watcher or not
    /// # Errors
    /// This method produces an error in the following cases:
    /// * `self.if_exists` is set to skip
    pub fn get_new_path(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(from.exists());
        assert!(self.to.exists());
        match self.if_exists {
            ConflictOption::Skip => Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("error: {} already exists", self.to.display()),
            )),
            ConflictOption::Overwrite => {
                if self.to.is_dir() {
                    return Ok(self.to.join(from.file_name().unwrap()));
                }
                Ok(self.to.clone())
            }
            ConflictOption::Rename => {
                let mut new_path = self.to.clone();
                if new_path.is_dir() {
                    new_path.push(from.file_name().unwrap())
                }
                let (stem, extension) = get_stem_and_extension(&new_path);
                let new_dir = new_path.parent().unwrap().to_path_buf();
                if new_path.exists() {
                    let mut n = 1;
                    while new_path.exists() {
                        let new_filename = format!("{}{}({:?}).{}", stem, self.counter_separator, n, extension);
                        new_path = new_dir.join(new_filename);
                        n += 1;
                    }
                }
                Ok(new_path)
            }
            ConflictOption::Ask => {
                assert_ne!(ConflictOption::default(), ConflictOption::Ask);
                let action = ConflictingFileOperation {
                    if_exists: if watching {
                        Default::default()
                    } else {
                        resolve_conflict(from, &self.to)
                    },
                    to: self.to.clone(),
                    counter_separator: self.counter_separator.clone(),
                };
                action.get_new_path(from, watching)
            }
        }
    }
}

/// Defines the options available to resolve a naming conflict,
/// i.e. how the application should proceed when a file exists
/// but it should move/rename/copy some file to that existing path
// write their configs with this format due to how serde deserializes files
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
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

impl ConflictOption {
    pub fn should_skip(&self) -> bool {
        self == &Self::Skip
    }

    pub fn should_overwrite(&self) -> bool {
        self == &Self::Overwrite
    }

    pub fn should_rename(&self) -> bool {
        self == &Self::Rename
    }

    pub fn should_ask(&self) -> bool {
        self == &Self::Ask
    }
}
