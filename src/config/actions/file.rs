use crate::config::actions::{Action, ConflictOption};
use crate::config::filters::Filters;
use crate::config::rules::Pattern;
use regex::Regex;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub struct File<'a> {
    path: &'a mut PathBuf,
    actions: Vec<Action>,
    conflict_option: ConflictOption,
}

impl<'a> File<'a> {
    pub fn from(path: &'a mut PathBuf) -> Result<Self, Error> {
        assert_eq!(path.is_file(), true, "ERROR: tried creating a config::actions::File from a non-file PathBuf");
        match path.file_name() {
            Some(_) => Ok(File {
                path,
                actions: vec![],
                conflict_option: ConflictOption::Rename,
            }),
            None => Err(Error::new(
                ErrorKind::InvalidInput,
                "ERROR: invalid path",
            )),
        }
    }

    pub fn copy(&self) -> Result<&Self, Error> {
        Ok(self)
    }

    fn move_to(&self, dst: &PathBuf) -> Result<&Self, Error> {
        assert!(dst.is_dir());
        let (stem, extension) = self.get_stem_and_extension(self.path)?;
        let mut new_path = dst.clone().join(format!("{}.{}", stem, extension));
        match self.conflict_option {
            ConflictOption::Rename => {
                if new_path.exists() {
                    let mut n = 1;
                    while new_path.exists() {
                        let new_filename = format!("{} ({:?}).{}", stem, n, extension);
                        new_path = dst.join(new_filename);
                        n += 1;
                    }
                }
                process(&new_path)
            }
            ConflictOption::Overwrite => process(&new_path),
            ConflictOption::Skip => {
                if new_path.exists() {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "ERROR: {} already exists",
                            new_path.to_str().ok_or_else(|| Error::new(ErrorKind::InvalidData, "ERROR: cannot convert OsStr to &str"))?),
                    ))
                }
                Ok(self)
            }
        }
    }

    fn get_stem_and_extension(self, new_name: &PathBuf) -> Result<(&str, &str), Error> {
        let stem = new_name
            .file_stem()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "file does not have a file stem (?)"))?
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to &str"))?;
        let extension = new_name
            .extension()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "file does not have an extension"))?
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to str"))?;

        Ok((stem, extension))
    }


    pub fn rename(&self, dst: &PathBuf) -> Result<&Self, Error> {
        let process = |new_path: &PathBuf| -> Result<&Self, Error> {
            let result = std::fs::rename(
                self.path.to_str().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "cannot convert PathBuf to &str")
                })?,
                new_path.to_str().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "cannot convert PathBuf to &str")
                })?,
            );
            match result {
                Ok(_) => Ok(self),
                Err(e) => Err(e),
            }
        };
        let (stem, extension) = self.get_stem_and_extension(dst)?;
        let new_path = if dst.is_file() {
            dst.parent()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "ERROR: invalid path"))?
        } else {
            dst.parent()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "ERROR: invalid path"))?
                .join(format!("{}.{}", stem, extension))
                .as_path()
        };
        let dst = if dst.is_file() {

        } else {

        };

        let mut new_path = dst.clone();
        match self.conflict_option {
            ConflictOption::Rename => {
                if new_path.exists() {
                    let mut n = 1;
                    while new_path.exists() {
                        let new_filename = format!("{} ({:?}).{}", stem, n, extension);
                        new_path = dst.join(new_filename);
                        n += 1;
                    }
                }
                process(&new_path)
            }
            ConflictOption::Overwrite => process(&new_path),
            ConflictOption::Skip => {
                if new_path.exists() {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "ERROR: {} already exists",
                            new_path.to_str().ok_or_else(|| Error::new(ErrorKind::InvalidData, "ERROR: cannot convert OsStr to &str"))?),
                    ))
                }
                Ok(self)
            }
        }
    }
    pub fn matches_filter(&self, filter: Filters) -> bool {}

    pub fn matches_pattern(&self, pattern: &Pattern) -> bool {
        let regex = &pattern.regex;
        let regex = Regex::new(regex).expect("ERROR: invalid regex");

        match self.path.to_str() {
            Some(path) => regex.is_match(path),
            None => false,
        }
    }
}
