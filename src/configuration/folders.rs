use crate::configuration::options::Options;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Folder {
    pub path: PathBuf,
    pub options: Options,
}
