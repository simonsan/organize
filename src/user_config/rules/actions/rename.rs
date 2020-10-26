use crate::user_config::rules::{
    actions::{ActionType, IOAction},
    deserialize::string_or_struct,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Result, ops::Deref, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Rename(#[serde(deserialize_with = "string_or_struct")] IOAction);

impl Deref for Rename {
    type Target = IOAction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rename {
    pub(super) fn run(&self, path: &mut Cow<Path>) -> Result<()> {
        IOAction::helper(path, self.deref(), ActionType::Rename)
    }
}