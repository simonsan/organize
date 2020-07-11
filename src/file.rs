use std::io::Error;
use std::path::{Path, PathBuf};
use yaml_rust::Yaml;
use regex::Regex;

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

    pub fn matches_pattern(&self, pattern: &'a Yaml) -> bool {
        let regex = pattern["regex"].as_str().unwrap();
        let regex = Regex::new(regex)
            .expect("ERROR: invalid regex");

        regex.is_match(self.path.to_str().unwrap())
    }
}
