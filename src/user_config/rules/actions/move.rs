use crate::user_config::rules::{
    actions::{ActionType, FileAction},
    deserialize::string_or_struct,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Result, ops::Deref, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Move(#[serde(deserialize_with = "string_or_struct")] FileAction);

impl Deref for Move {
    type Target = FileAction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Move {
    pub(super) fn run(&self, path: &mut Cow<Path>) -> Result<()> {
        let r#move = self.deref();
        FileAction::helper(path, &r#move.to, &r#move.if_exists, &r#move.sep, ActionType::Move)
    }
}
