use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind, Result},
    ops::Deref,
    path::Path,
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Trash(bool);

impl Deref for Trash {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Trash {
    pub(super) fn run(&self, path: &Path) -> Result<()> {
        if self.0 {
            return match trash::delete(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
            };
        }
        Ok(())
    }
}
