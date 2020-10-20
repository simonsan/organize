use crate::{string::Placeholder, user_config::rules::actions::shell::Shell};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Python(String);

impl Python {
    fn write(&self, path: &Path) -> Result<PathBuf> {
        let dir = Shell::dir();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        let script = Shell::path((&path.file_name().unwrap()).as_ref());
        fs::write(&script, self.0.expand_placeholders(path).unwrap())?;
        Ok(script)
    }

    pub fn run(&self, path: &Path) -> Result<()> {
        let script = self.write(&path)?;
        let output = Command::new("python")
            .arg(Shell::path(&path))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("could not run shell script")
            .wait_with_output()
            .expect("shell script terminated with an error");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        fs::remove_file(script)?;
        Ok(())
    }
}
