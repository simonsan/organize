use regex::Regex;
use std::io::Error;
use std::path::{Path, PathBuf};
use yaml_rust::Yaml;

pub struct File<'a> {
    path: &'a PathBuf,
}

impl<'a> File<'a> {
    pub fn from(path: &PathBuf) -> File {
        File { path }
    }

    pub fn rename(&self, dst: &str) -> Result<(), Error> {
        let file_stem = self.path.file_stem().unwrap().to_str().unwrap();
        let extension = self.path.extension().unwrap().to_str().unwrap();
        let mut new_path = Path::new(dst).join(format!("{}.{}", file_stem, extension));
        let mut n = 1;

        while new_path.exists() {
            let new_filename = format!(
                "{} ({:?}).{}",
                file_stem,
                n,
                extension
            );
            new_path = Path::new(dst).join(new_filename);
            n += 1;
        }

        Ok(std::fs::rename(
            self.path.to_str().unwrap(),
            new_path.to_str().unwrap(),
        )?)
    }

    pub fn matches_pattern(&self, pattern: &'a Yaml) -> bool {
        let regex = pattern["regex"].as_str().unwrap();
        let regex = Regex::new(regex).expect("ERROR: invalid regex");

        regex.is_match(self.path.to_str().unwrap())
    }
}
