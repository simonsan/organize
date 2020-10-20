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
    io::Result,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Copy {
    #[serde(deserialize_with = "deserialize_path")]
    pub to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub sep: Sep,
}

impl Copy {
    pub fn run(&self, path: &Path) -> Result<Option<PathBuf>> {
        let mut logger = Logger::default();
        let mut to: PathBuf = self.to.to_str().unwrap().to_string().expand_placeholders(path)?.into();
        if !to.exists() {
            fs::create_dir_all(&to)?;
        }
        to = to.join(&path.file_name().unwrap());

        if to.exists() {
            if let Some(to) = to.update(&self.if_exists, &self.sep) {
                std::fs::copy(&path, &to)?;
                logger.try_write(
                    Level::Info,
                    Action::Copy,
                    &format!("{} -> {}", &path.display(), &to.display()),
                );
                Ok(Some(to))
            } else {
                Ok(None)
            }
        } else {
            std::fs::copy(&path, &to)?;
            logger.try_write(
                Level::Info,
                Action::Copy,
                &format!("{} -> {}", &path.display(), &to.display()),
            );
            Ok(Some(to))
        }
    }
}
