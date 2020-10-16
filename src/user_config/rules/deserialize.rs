use core::result::Result::Ok;
use std::path::PathBuf;

use regex::Regex;
use serde::{
    Deserialize,
    Deserializer,
};

use crate::path::Expandable;

pub(in crate::user_config) fn deserialize_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    Ok(PathBuf::from(&buf).fullpath())
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

pub(in crate::user_config) fn default_sep() -> String {
    " ".to_string()
}
