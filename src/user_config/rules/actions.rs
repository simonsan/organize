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
    file::File,
    utils::new_filepath,
};

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
            self.copy(&file.path, watching)?;
        }
        // TODO the following three are conflicting operations - validate this
        if self.r#move.is_some() {
            let old_path = file.path.clone();
            file.path = self.r#move(&file.path, watching)?;
            println!("moved {} to {}", old_path.display(), file.path.display());
        }

        if self.rename.is_some() {
            file.path = self.rename(&file.path, watching)?;
        }
        if self.delete.is_some() {
            self.delete(&file.path)?;
        }
        Ok(())
    }

    fn copy(&self, from: &Path, watching: bool) -> Result<(), Error> {
        assert!(self.copy.is_some()); // should check that it's some before calling this method
        let copy = self.copy.as_ref().unwrap();
        if copy.if_exists == ConflictOption::Skip || from == copy.to {
            return Ok(());
        }

        let dst = new_filepath(from, &copy, watching)?;
        std::fs::copy(from, dst.as_path()).expect("cannot write file (permission denied)");
        Ok(())
    }

    fn rename(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(self.rename.is_some()); // should check that it's some before calling this method
        let rename = self.rename.as_ref().unwrap();

        if rename.if_exists == ConflictOption::Skip || from == rename.to {
            return Ok(from.to_path_buf());
        }
        let dst = new_filepath(from, &rename, watching)?;
        std::fs::rename(from, &dst).expect("couldn't rename file");
        Ok(dst)
    }

    fn r#move(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(self.r#move.is_some()); // should check that it's some before calling this method
        let r#move = self.r#move.as_ref().unwrap();

        if r#move.if_exists == ConflictOption::Skip || from == r#move.to {
            return Ok(from.to_path_buf());
        }
        if !r#move.to.exists() {
            fs::create_dir_all(&r#move.to)?;
        }

        let dst = new_filepath(from, &r#move, watching)?;

        std::fs::rename(from, dst.as_path()).expect("couldn't rename file");
        Ok(dst)
    }

    fn delete(&self, path: &Path) -> Result<(), Error> {
        std::fs::remove_file(path)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConflictingFileOperation {
    #[serde(deserialize_with = "deserialize_path")]
    pub to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub counter_separator: String,
}

impl From<&str> for ConflictingFileOperation {
    fn from(path: &str) -> Self {
        let mut op = ConflictingFileOperation::default();
        op.to = PathBuf::from(path);
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
            counter_separator: " ".to_string(),
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
