use crate::user_config::rules::{
    actions::{file_action::FileAction, Action},
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
        let rename = self.deref();
        FileAction::helper(path, &rename.to, &rename.if_exists, &rename.sep, Action::Rename)
    }
}
