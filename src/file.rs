use crate::config::Pattern;
use regex::Regex;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub struct File<'a> {
    path: &'a PathBuf,
}

impl<'a> File<'a> {
    pub fn from(path: &PathBuf) -> File {
        File { path }
    }

    pub fn rename(self, dst: String) -> Result<Self, Error> {
        let stem = self
            .path
            .file_stem()
            .ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "file does not have a file stem (?)")
            })?
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to &str"))?;
        let extension = self
            .path
            .extension()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "file does not have an extension"))?
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to str"))?;

        let mut new_path = Path::new(&dst).join(format!("{}.{}", stem, extension));
        let mut n = 1;

        while new_path.exists() {
            let new_filename = format!("{} ({:?}).{}", stem, n, extension);
            new_path = Path::new(&dst).join(new_filename);
            n += 1;
        }

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
    }

    pub fn matches_pattern(&self, pattern: &Pattern) -> bool {
        let regex = &pattern.regex;
        let regex = Regex::new(regex).expect("ERROR: invalid regex");

        match self.path.to_str() {
            Some(path) => regex.is_match(path),
            None => false,
        }
    }
}
