use crate::path::Expandable;
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

pub(in crate::user_config) fn deserialize_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let path = PathBuf::from(&buf).expand_vars();
    if !path.exists() {
        fs::create_dir_all(&path).expect("error: declared non-existing directory that could not be created");
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
