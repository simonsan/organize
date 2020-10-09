use crate::configuration::options::Options;
use clap::ErrorKind;
use serde::Deserialize;
use std::{
    io::Error,
    ops::Index,
    path::{
        Path,
        PathBuf,
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct Folder {
    pub path: PathBuf,
    pub options: Option<Options>,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            options: Some(Default::default()),
        }
    }
}

type Folders = Vec<Folder>;
