use crate::configuration::options::Options;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Folder {
    pub path: PathBuf,
    pub options: Options,
}
