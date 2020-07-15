pub mod utils;

use crate::configuration::actions::ConflictOption;
use crate::file::utils::new_filepath;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
pub struct File {
    pub path: PathBuf,
}

impl<'a> File {
    pub fn from(s: &str) -> Result<Self, Error> {
        let path = canonicalize(s)?;
        if !path.exists() || !path.is_file() || path.parent().is_none() {
            return Err(Error::from(ErrorKind::InvalidInput));
        }

        Ok(File {
            path,
        })
    }

    pub fn copy(&self, to: &Path, conflict_option: ConflictOption) -> Result<(), Error> {
        let new_path = new_filepath(&self.path, to, conflict_option)?;
        std::fs::copy(self.path.as_path(), new_path.as_path()).expect("cannot write file (permission denied)");
        Ok(())
    }

    // works for move too
    pub fn rename(&self, to: &Path, conflict_option: ConflictOption) -> Result<(), Error> {
        let new_path = new_filepath(&self.path, &to, conflict_option)?;
        std::fs::rename(self.path.as_path(), new_path.as_path()).expect("couldn't rename file");
        Ok(())
    }

    pub fn delete(&self) -> Result<(), Error> {
        std::fs::remove_file(&self.path)
    }
}
