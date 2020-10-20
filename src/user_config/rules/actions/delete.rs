use serde::{Deserialize, Serialize};
use std::{fs, io::Result, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Delete(bool);

impl Delete {
    pub fn as_bool(&self) -> bool {
        self.0
    }

    pub fn run(&self, path: &Path) -> Result<()> {
        fs::remove_file(path)
    }
}
