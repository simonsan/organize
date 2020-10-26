use crate::{string::Placeholder, user_config::UserConfig};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Script {
    exec: String,
    content: String,
}

impl Script {
    pub fn write(&self, path: &Path) -> Result<PathBuf> {
        let content = self.content.expand_placeholders(path)?;
        let dir = UserConfig::dir().join("scripts");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let script = dir.join("temp_script");
        fs::write(&script, content)?;
        Ok(script)
    }

    pub fn run(&self, path: &Path) -> Result<()> {
        let script = self.write(path)?;
        let output = Command::new(&self.exec)
            .arg(&script)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("could not run script")
            .wait_with_output()
            .expect("script terminated with an error");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        fs::remove_file(script)?;
        Ok(())
    }
}
