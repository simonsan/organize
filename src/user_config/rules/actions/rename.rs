use super::deserialize_path;
use crate::{
    path::Update,
    string::Placeholder,
    subcommands::logs::{Level, Logger},
    user_config::rules::actions::{Action, ConflictOption, Sep},
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Rename {
    #[serde(deserialize_with = "deserialize_path")]
    pub to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub sep: Sep,
}

impl Rename {
    pub fn run(&self, path: &Path, is_watching: bool) -> Result<Option<PathBuf>, Error> {
        let mut logger = Logger::default();
        let to: PathBuf = self.to.to_str().unwrap().to_string().expand_placeholders(path)?.into();
        if to.exists() {
            if let Some(to) = to.update(&self.if_exists, &self.sep, is_watching) {
                std::fs::rename(&path, &to)?;
                logger.try_write(
                    Level::Info,
                    Action::Rename,
                    &format!("{} -> {}", &path.display(), &to.display()),
                );
                Ok(Some(to))
            } else {
                Ok(None)
            }
        } else {
            std::fs::rename(&path, &to)?;
            logger.try_write(
                Level::Info,
                Action::Rename,
                &format!("{} -> {}", &path.display(), &to.display()),
            );
            Ok(Some(to))
        }
    }
}
