use crate::{
    file::File,
    utils::expand_env_vars,
};
use core::result::Result::Ok;
use regex::Regex;
use serde::{
    Deserialize,
    Deserializer,
};
use std::path::{
    Path,
    PathBuf,
};

pub fn deserialize_path<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PathBuf, D::Error> {
    let buf = String::deserialize(deserializer)?;
    if buf.is_empty() {
        panic!("could not parse config file: field 'to' must represent a valid path")
    }
    Ok(expand_env_vars(Path::new(&buf)))
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
