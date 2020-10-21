use super::deserialize::deserialize_path;
use crate::{
    path::Update,
    string::Placeholder,
    subcommands::logs::{Level, Logger},
    user_config::UserConfig,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Sep(String);

impl Default for Sep {
    fn default() -> Self {
        Self(" ".into())
    }
}

impl Sep {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Eq, PartialEq)]
pub enum Action {
    Move,
    Rename,
    Copy,
    Delete,
    Trash,
    Echo,
    Shell,
    Python,
}

impl From<&str> for Action {
    fn from(str: &str) -> Self {
        match str.to_lowercase().as_str() {
            "move" => Self::Move,
            "copy" => Self::Copy,
            "rename" => Self::Rename,
            "delete" => Self::Delete,
            "trash" => Self::Trash,
            "echo" => Self::Echo,
            "shell" => Self::Shell,
            "python" => Self::Python,
            _ => panic!("unknown action"),
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Self::Move => "move",
            Self::Copy => "copy",
            Self::Rename => "rename",
            Self::Delete => "delete",
            Self::Trash => "trash",
            Self::Echo => "echo",
            Self::Shell => "shell",
            Self::Python => "python",
        }
        .into()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct FileAction {
    #[serde(deserialize_with = "deserialize_path")]
    pub to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub sep: Sep,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Actions {
    pub echo: Option<String>,
    pub shell: Option<String>,
    pub python: Option<String>,
    pub delete: Option<bool>,
    pub copy: Option<FileAction>,
    pub r#move: Option<FileAction>,
    pub rename: Option<FileAction>,
}

impl Actions {
    pub fn run(&self, mut path: PathBuf) -> Result<()> {
        assert!((self.r#move.is_some() ^ self.rename.is_some()) || self.r#move.is_none() && self.rename.is_none());
        if self.echo.is_some() {
            self.echo(&path);
        }
        if self.python.is_some() {
            self.python(&path)?;
        }
        if self.shell.is_some() {
            self.shell(&path)?;
        }
        if self.copy.is_some() {
            self.copy(&path)?;
        }
        if self.r#move.is_some() ^ self.rename.is_some() {
            let mut new_path = PathBuf::new();
            if self.r#move.is_some() {
                if let Some(path) = self.r#move(&path)? {
                    new_path = path;
                }
            } else if self.rename.is_some() {
                if let Some(path) = self.rename(&path)? {
                    new_path = path;
                }
            }
            path = new_path;
        }
        if self.delete.is_some() && self.delete.unwrap() {
            self.delete(&path)?;
        }
        Ok(())
    }

    fn delete(&self, path: &Path) -> Result<()> {
        fs::remove_file(path)
    }

    fn copy(&self, path: &Path) -> Result<Option<PathBuf>> {
        assert!(self.copy.is_some());
        let copy = self.copy.as_ref().unwrap();
        Self::helper(path, &copy.to, &copy.if_exists, &copy.sep, Action::Copy)
    }

    fn rename(&self, path: &Path) -> Result<Option<PathBuf>> {
        assert!(self.rename.is_some());
        let rename = self.rename.as_ref().unwrap();
        Self::helper(path, &rename.to, &rename.if_exists, &rename.sep, Action::Rename)
    }

    fn r#move(&self, path: &Path) -> Result<Option<PathBuf>> {
        assert!(self.r#move.is_some());
        let r#move = self.r#move.as_ref().unwrap();
        Self::helper(path, &r#move.to, &r#move.if_exists, &r#move.sep, Action::Move)
    }

    fn echo(&self, path: &Path) {
        assert!(self.echo.is_some());
        let echo = self.echo.as_ref().unwrap();
        println!("{}", echo.expand_placeholders(path).unwrap());
    }

    fn shell(&self, path: &Path) -> Result<()> {
        assert!(self.shell.is_some());
        let shell = self.shell.as_ref().unwrap();
        let script = Self::write_script(shell, path)?;
        let output = Command::new("sh")
            .arg(&script)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("could not run shell script")
            .wait_with_output()
            .expect("shell script terminated with an error");
        fs::remove_file(script)?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    fn python(&self, path: &Path) -> Result<()> {
        assert!(self.python.is_some());
        let python = self.python.as_ref().unwrap();
        let script = Self::write_script(python, path)?;
        let output = Command::new("python")
            .arg(&script)
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

    fn write_script(content: &str, path: &Path) -> Result<PathBuf> {
        let dir = UserConfig::dir().join("scripts");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let script = dir.join(path.file_name().unwrap());
        fs::write(&script, content.to_string().expand_placeholders(path).unwrap())?;
        Ok(script)
    }

    fn helper(
        path: &Path,
        to: &Path,
        if_exists: &ConflictOption,
        sep: &Sep,
        r#type: Action,
    ) -> Result<Option<PathBuf>> {
        assert!(r#type == Action::Move || r#type == Action::Rename || r#type == Action::Copy);
        let mut logger = Logger::default();
        let mut to: PathBuf = to.to_str().unwrap().to_string().expand_placeholders(path)?.into();
        if r#type == Action::Copy || r#type == Action::Move {
            if !to.exists() {
                fs::create_dir_all(&to)?;
            }
            to = to.join(&path.file_name().unwrap());
            println!("{}", to.display())
        }
        if to.exists() {
            if let Some(new_path) = to.update(&if_exists, &sep) {
                to = new_path;
            } else {
                return Ok(None);
            }
        }
        if r#type == Action::Copy {
            std::fs::copy(&path, &to)?;
        } else if r#type == Action::Rename || r#type == Action::Move {
            std::fs::rename(&path, &to)?;
        }
        logger.try_write(
            Level::Info,
            r#type,
            &format!("{} -> {}", &path.display(), &to.display()),
        );
        Ok(Some(to))
    }
}

/// Defines the options available to resolve a naming conflict,
/// i.e. how the application should proceed when a file exists
/// but it should move/rename/copy some file to that existing path
// write their configs with this format due to how serde deserializes files
#[derive(Eq, PartialEq, Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum ConflictOption {
    Overwrite,
    Skip,
    Rename,
    Ask, // not available when watching
}

impl Default for ConflictOption {
    fn default() -> Self {
        ConflictOption::Rename
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind, Result};

    use crate::{
        path::{
            lib::vars::{expected_path, test_file_or_dir},
            Update,
        },
        user_config::rules::actions::ConflictOption,
    };

    #[test]
    fn rename_with_rename_conflict() -> Result<()> {
        let target = test_file_or_dir("test2.txt");
        let expected = expected_path(&target, &Default::default());
        let new_path = target.update(&ConflictOption::Rename, &Default::default()).unwrap();
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_overwrite_conflict() -> Result<()> {
        let target = test_file_or_dir("test2.txt");
        let expected = target.clone();
        let new_path = target.update(&ConflictOption::Overwrite, &Default::default()).unwrap();
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    #[should_panic] // unwrapping a None value
    fn rename_with_skip_conflict() {
        let target = test_file_or_dir("test2.txt");
        target.update(&ConflictOption::Skip, &Default::default()).unwrap();
    }

    #[test]
    #[should_panic] // trying to modify a path that does not exist
    fn new_path_to_non_existing_file() {
        let target = test_file_or_dir("test_dir2").join("test1.txt");
        assert!(!target.exists());
        target.update(&ConflictOption::Rename, &Default::default()).unwrap();
    }
}
