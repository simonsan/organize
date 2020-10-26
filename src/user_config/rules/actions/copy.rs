use crate::user_config::rules::{
    actions::{Action, FileAction},
    deserialize::string_or_struct,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Result, ops::Deref, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Copy(#[serde(deserialize_with = "string_or_struct")] FileAction);

impl Deref for Copy {
    type Target = FileAction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Copy {
    pub(super) fn run(&self, path: &mut Cow<Path>) -> Result<()> {
        let copy = self.deref();
        FileAction::helper(path, &copy.to, &copy.if_exists, &copy.sep, Action::Copy)
    }
}
