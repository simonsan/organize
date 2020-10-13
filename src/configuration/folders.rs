use crate::configuration::options::Options;
use serde::{
    Deserialize,
    Serialize,
};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Folder {
    pub path: PathBuf,
    #[serde(default)]
    pub options: Options,
}
