use crate::configuration::options::Options;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Folder {
    pub path: Option<PathBuf>,
    pub options: Option<Options>,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            path: None,
            options: Some(Default::default()),
        }
    }
}
