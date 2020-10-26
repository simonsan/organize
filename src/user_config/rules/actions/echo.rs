use crate::string::Placeholder;
use serde::{Deserialize, Serialize};
use std::{io::Result, ops::Deref, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Echo(String);

impl Deref for Echo {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Echo {
    pub(super) fn run(&self, path: &Path) -> Result<()> {
        println!("{}", self.deref().expand_placeholders(path)?);
        Ok(())
    }
}
