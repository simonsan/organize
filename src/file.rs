use crate::config::Rule;
use regex::Regex;
use std::io::Error;
use std::path::{Path, PathBuf};

pub struct File<'a> {
    path: &'a PathBuf,
}

impl<'a> File<'a> {
    pub fn from(path: &PathBuf) -> File {
        File { path }
    }

    pub fn rename(&self, dst: &str) -> Result<(), Error> {
        Ok(std::fs::rename(
            self.path.to_str().unwrap(),
            Path::new(dst)
                .join(self.path.file_name().unwrap())
                .to_str()
                .unwrap(),
        )?)
    }
}
