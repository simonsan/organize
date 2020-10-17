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

use crate::path::{
    Expandable,
    Update,
};

use super::deserialize::{
    default_sep,
    deserialize_path,
};

mod lib;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Actions {
    pub echo: Option<String>,
    pub shell: Option<String>,
    pub trash: Option<bool>,
    pub delete: Option<bool>,
    pub copy: Option<ConflictingFileOperation>,
    pub r#move: Option<ConflictingFileOperation>,
    pub rename: Option<ConflictingFileOperation>,
}

impl Actions {
    pub fn run(&self, mut path: PathBuf, watching: bool) -> Result<(), Error> {
        assert!(self.r#move.is_some() ^ self.rename.is_some());
        if self.copy.is_some() {
            self.copy(&path, watching)?;
        }
        if self.r#move.is_some() ^ self.rename.is_some() {
            let mut result = PathBuf::new();
            if self.r#move.is_some() {
                if let Some(path) = self.r#move(&path, watching)? {
                    result = path;
                }
            } else if let Some(path) = self.rename(&path, watching)? {
                result = path;
            }
            path = result;
        }
        if self.delete.is_some() {
            self.delete(&path)?;
        }
        Ok(())
    }

    fn copy(&self, path: &Path, is_watching: bool) -> Result<Option<PathBuf>, Error> {
        assert!(self.copy.is_some());
        let copy = self.copy.as_ref().unwrap();
        if !copy.to.exists() {
            fs::create_dir_all(&copy.to)?;
        }
        let to = copy.to.join(&path.file_name().unwrap());
        if to.exists() {
            if let Some(to) = to.update(&copy.if_exists, &copy.sep, is_watching) {
                std::fs::copy(&path, &to)?;
                Ok(Some(to))
            } else {
                Ok(None)
            }
        } else {
            std::fs::copy(&path, &to)?;
            Ok(Some(to))
        }
    }

    fn rename(&self, path: &Path, is_watching: bool) -> Result<Option<PathBuf>, Error> {
        assert!(self.rename.is_some());
        let rename = self.rename.as_ref().unwrap();
        if rename.to.exists() {
            if let Some(to) = rename.to.update(&rename.if_exists, &rename.sep, is_watching) {
                std::fs::rename(&path, &to)?;
                Ok(Some(to))
            } else {
                Ok(None)
            }
        } else {
            std::fs::rename(&path, &rename.to)?;
            Ok(Some(rename.to.clone()))
        }
    }

    fn r#move(&self, path: &Path, is_watching: bool) -> Result<Option<PathBuf>, Error> {
        assert!(self.r#move.is_some());
        let r#move = self.r#move.as_ref().unwrap();
        if !r#move.to.exists() {
            fs::create_dir_all(&r#move.to)?;
        }
        let to = r#move.to.join(&path.file_name().unwrap());
        if to.exists() {
            if let Some(to) = to.update(&r#move.if_exists, &r#move.sep, is_watching) {
                std::fs::rename(&path, &to)?;
                Ok(Some(to))
            } else {
                Ok(None)
            }
        } else {
            std::fs::rename(&path, &to)?;
            Ok(Some(to))
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
    #[serde(default = "default_sep")]
    pub sep: String,
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
        op.to = path;
        op
    }
}

impl Default for ConflictingFileOperation {
    fn default() -> Self {
        Self {
            to: PathBuf::new(), // shouldn't get to this if 'to' isn't specified
            if_exists: Default::default(),
            sep: " ".to_string(),
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
