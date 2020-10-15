use core::result::Result::Ok;
use regex::Regex;
use serde::{
    Deserialize,
    Deserializer,
};
use std::{
    env,
    fs,
    path::{
        Path,
        PathBuf,
    },
};

pub trait PathExpandable<T> {
    fn expand(&self) -> T;
}

impl PathExpandable<PathBuf> for PathBuf {
    fn expand(&self) -> PathBuf {
        self.components()
            .map(|comp| {
                let path: &Path = comp.as_ref();
                let path = path.to_str().unwrap();
                if path.starts_with('$') {
                    env::var(path.replace('$', ""))
                        .unwrap_or_else(|_| panic!("error: environment variable '{}' could not be found", path))
                } else {
                    path.to_string()
                }
            })
            .collect()
    }
}

pub fn deserialize_path<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PathBuf, D::Error> {
    let buf = String::deserialize(deserializer)?;
    if buf.is_empty() {
        panic!("could not parse config file: field 'to' must represent a valid path")
    }
    let mut path = PathBuf::from(&buf).expand();
    if path.is_relative() {
        panic!("could not parse config file: relative paths not allowed")
    }
    if !path.exists() {
        fs::create_dir_all(&path).expect("error: declared non-existent directory in config that could not be created");
    }
    Ok(path)
}

pub fn deserialize_regex<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Regex, D::Error> {
    let buf = String::deserialize(deserializer)?;
    let regex = Regex::new(&buf).expect("error: could not parse config file (invalid regex)");
    Ok(regex)
}

#[allow(clippy::trivial_regex)]
pub fn default_regex() -> Regex {
    Regex::new("").unwrap()
}
