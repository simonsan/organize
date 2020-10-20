use crate::{string::Placeholder, user_config::UserConfig};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    fs::OpenOptions,
    io::{Result, Stdout},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Shell(String);

impl Shell {
    fn dir() -> PathBuf {
        UserConfig::dir().join("scripts")
    }

    fn path(path: &Path) -> PathBuf {
        Self::dir().join(path.file_name().unwrap())
    }

    fn write(&self, path: &Path) -> Result<PathBuf> {
        let dir = Self::dir();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        let script = Self::path((&path.file_name().unwrap()).as_ref());
        fs::write(Self::path(&script), self.0.expand_placeholders(path).unwrap())?;
        Ok(script)
    }

    pub fn run(&self, path: &Path) -> Result<()> {
        let script = self.write(&path)?;
        Command::new("sh")
            .arg(Self::path(&path))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("could not run shell script")
            .wait_with_output()
            .expect("shell script terminated with an error");
        fs::remove_file(script)?;
        Ok(())
    }
}
