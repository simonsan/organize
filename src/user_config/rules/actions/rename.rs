use crate::user_config::rules::{
    actions::{ActionType, FileAction},
    deserialize::string_or_struct,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Result, ops::Deref, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Rename(#[serde(deserialize_with = "string_or_struct")] FileAction);

impl Deref for Rename {
    type Target = FileAction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rename {
    pub(super) fn run(&self, path: &mut Cow<Path>) -> Result<()> {
        FileAction::helper(path, self.deref(), ActionType::Rename)
    }
}
