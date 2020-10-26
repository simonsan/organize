use crate::user_config::rules::{
    actions::{ActionType, IOAction},
    deserialize::string_or_struct,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Result, ops::Deref, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Copy(#[serde(deserialize_with = "string_or_struct")] IOAction);

impl Deref for Copy {
    type Target = IOAction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Copy {
    pub(super) fn run(&self, path: &mut Cow<Path>) -> Result<()> {
        IOAction::helper(path, self.deref(), ActionType::Copy)
    }
}
