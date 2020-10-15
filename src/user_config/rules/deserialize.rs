use crate::path::Expand;
use core::result::Result::Ok;
use regex::Regex;
use serde::{
    Deserialize,
    Deserializer,
};
use std::{
    fs,
    path::PathBuf,
};

pub(in crate::user_config) fn deserialize_path<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<PathBuf, D::Error> {
    let buf = String::deserialize(deserializer)?;
    if buf.is_empty() {
        panic!("could not parse config file: field 'to' must represent a valid path")
    }
    let path = PathBuf::from(&buf).expand();
    if path.is_relative() {
        panic!("could not parse config file: relative paths not allowed")
    }
    if !path.exists() {
        fs::create_dir_all(&path).expect("error: declared non-existent directory in config that could not be created");
    }
    Ok(path)
}

pub(in crate::user_config) fn deserialize_regex<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Regex, D::Error> {
    let buf = String::deserialize(deserializer)?;
    let regex = Regex::new(&buf).expect("error: could not parse config file (invalid regex)");
    Ok(regex)
}

#[allow(clippy::trivial_regex)]
pub(in crate::user_config) fn default_regex() -> Regex {
    Regex::new("").unwrap()
}
