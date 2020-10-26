use crate::user_config::rules::actions::Script;
use serde::{Deserialize, Serialize};
use std::{io::Result, ops::Deref, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Shell(Script);

impl Deref for Shell {
    type Target = Script;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Shell {
    pub(super) fn run(&self, path: &Path) -> Result<()> {
        let shell = self.deref();
        shell.run(path, "sh")
    }
}
