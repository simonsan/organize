use crate::config::actions::{Action, ConflictOption};
use crate::config::filters::Filters;
use crate::config::rules::Pattern;
use regex::Regex;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use crate::config::actions::utils::{get_stem_and_extension};


pub struct File {
    path: PathBuf,
}

impl<'a> File {
    pub fn from(path: &'a mut PathBuf) -> Result<Self, Error> {
        assert!(
            path.is_file(),
            "ERROR: tried creating a config::actions::File from a non-file PathBuf"
        );
        assert!(
            path.parent().is_some(),
            "ERROR: file does not exist within a valid directory"
        );
        match path.file_name() {
            Some(_) => Ok(File {
                path: path.to_path_buf(),
            }),
            None => Err(Error::new(ErrorKind::InvalidInput, "ERROR: invalid path")),
        }
    }

    pub fn copy(&self) -> Result<&Self, Error> {
        Ok(self)
    }

    /// Checks whether a given target std::dst::PathBuf exists or not and decides what changes are needed based on `conflict_option`
    /// # Arguments
    /// * `stem`: stem of the original file
    /// * `extension`: extension of the original file
    /// * `dst`: target filepath to check for
    /// * `conflict_option`: options to modify the target filepath
    /// # Returns
    /// Returns a std::io::Error if `conflict_option != ConflictOption::Skip`, otherwise returns a possibly altered PathBuf
    fn solve_conflicts(&self, stem: &str, extension: &str, dst: &PathBuf, conflict_option: ConflictOption) -> Result<PathBuf, Error> {
        let new_dir = if dst.is_file() {
            dst.parent().ok_or_else(|| {
                Error::new(ErrorKind::InvalidInput,"ERROR: invalid parent directory for target location")
            })?
        } else {
            dst
        };

        let mut new_path = if dst.is_file() {
            dst.clone()
        } else {
            dst.join(format!("{}.{}", stem, extension))
        };

        if new_path.exists() && conflict_option == ConflictOption::Skip {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("ERROR: {} already exists",
                        new_path
                            .to_str()
                            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "ERROR: cannot convert OsStr to &str"))?
                ),
            ))
        }
        let mut n = 1;
        while new_path.exists() && conflict_option == ConflictOption::Rename {
            let new_filename = format!("{} ({:?}).{}", stem, n, extension);
            new_path = new_dir.join(new_filename);
            n += 1;
        }
        Ok(new_path)
    }

    pub fn r#move(&mut self, dst: &PathBuf, conflict_option: ConflictOption) -> Result<&Self, Error> {
        assert!(dst.is_dir());
        let (stem, extension) = get_stem_and_extension(&self.path)?;
        let new_path = self.solve_conflicts(stem, extension, dst, conflict_option)?;
        std::fs::rename(self.path.as_path(), dst.as_path())?;
        self.path = new_path;
        Ok(self)


    }

    pub fn rename(&mut self, dst: &PathBuf, conflict_option: ConflictOption) -> Result<&Self, Error> {
        assert!(dst.is_file());
        let (stem, extension) = get_stem_and_extension(dst)?;
        let new_path = self.solve_conflicts(stem, extension, dst, conflict_option)?;
        std::fs::rename(self.path.as_path(), dst.as_path())?;
        self.path = new_path;
        Ok(self)
    }
}
