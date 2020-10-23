use super::deserialize::deserialize_path;
use crate::{
    error::ErrorKind,
    path::Update,
    string::Placeholder,
    subcommands::logs::{Level, Logger},
    user_config::UserConfig,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fs,
    io::{Error, ErrorKind, Result},
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
    pub fn run(&self, path: PathBuf) {
        let mut path = Cow::from(path);
        if self.echo.is_some() {
            if let Err(e) = self.echo(&path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if self.python.is_some() {
            if let Err(e) = self.python(&path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if self.shell.is_some() {
            if let Err(e) = self.shell(&path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if self.copy.is_some() {
            if let Err(e) = self.copy(&mut path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if self.r#move.is_some() ^ self.rename.is_some() {
            let mut result = if self.r#move.is_some() {
                self.r#move(&mut path)
            } else {
                self.rename(&mut path)
            };
            if let Err(e) = result {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if self.delete.is_some() && self.delete.unwrap() {
            if let Err(e) = self.delete(&path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
    }

    fn delete(&self, path: &Path) -> Result<()> {
        fs::remove_file(path)
    }

    fn copy(&self, path: &mut Cow<Path>) -> Result<()> {
        debug_assert!(self.copy.is_some());
        let copy = self.copy.as_ref().unwrap();
        Self::helper(path, &copy.to, &copy.if_exists, &copy.sep, Action::Copy)
    }

    fn rename(&self, path: &mut Cow<Path>) -> Result<()> {
        debug_assert!(self.rename.is_some());
        let rename = self.rename.as_ref().unwrap();
        Self::helper(path, &rename.to, &rename.if_exists, &rename.sep, Action::Rename)
    }

    fn r#move(&self, path: &mut Cow<Path>) -> Result<()> {
        debug_assert!(self.r#move.is_some());
        let r#move = self.r#move.as_ref().unwrap();
        Self::helper(path, &r#move.to, &r#move.if_exists, &r#move.sep, Action::Move)
    }

    fn echo(&self, path: &Path) -> Result<()> {
        debug_assert!(self.echo.is_some());
        let echo = self.echo.as_ref().unwrap();
        println!("{}", echo.expand_placeholders(path)?);
        Ok(())
    }

    fn shell(&self, path: &Path) -> Result<()> {
        debug_assert!(self.shell.is_some());
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
        debug_assert!(self.python.is_some());
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
        let script = dir.join("temp_script");
        let content = content.expand_placeholders(path)?;
        fs::write(&script, content)?;
        Ok(script)
    }

    /// Helper function for the move, rename and copy actions.
    /// # Args:
    /// - `path`: a reference to a Cow smart pointer containing the original file
    /// - `to`: the destination path of `path`
    /// - `if_exists`: variable that helps resolve any naming conflicts between `path` and `to`
    /// - `sep`: counter separator (e.g. in "file (1).test", `sep` would be a whitespace; in "file-(1).test", it would be a hyphen)
    /// - `type`: whether this helper should move, rename or copy the given file (`path`)
    fn helper(path: &mut Cow<Path>, to: &Path, if_exists: &ConflictOption, sep: &Sep, r#type: Action) -> Result<()> {
        #[cfg(debug_assertions)]
        debug_assert!(r#type == Action::Move || r#type == Action::Rename || r#type == Action::Copy);

        let mut logger = Logger::default();
        let to = PathBuf::from(to.to_str().unwrap().to_string().expand_placeholders(path).unwrap());
        let mut to = Cow::from(to);
        if r#type == Action::Copy || r#type == Action::Move {
            if !to.exists() {
                fs::create_dir_all(&to)?;
            }
            to.to_mut().push(
                path.file_name()
                    .ok_or_else(|| Error::new(ErrorKind::Other, "path has no filename"))?,
            );
        }

        if to.exists() && to.update(&if_exists, &sep).is_err() {
            Ok(())
        }

        if r#type == Action::Copy {
            std::fs::copy(&path, &to)?;
        } else if r#type == Action::Rename || r#type == Action::Move {
            std::fs::rename(&path, &to)?;
        }

        logger.try_write(
            &Level::Info,
            &r#type,
            &format!("{} -> {}", &path.display(), &to.display()),
        );

        if r#type == Action::Rename || r#type == Action::Move {
            *path = to;
        }
        Ok(())
    }
}

/// Defines the options available to resolve a naming conflict,
/// i.e. how the application should proceed when a file exists
/// but it should move/rename/copy some file to that existing path
#[derive(Eq, PartialEq, Debug, Clone, Copy, Deserialize, Serialize)]
// for the config schema to keep these options as lowercase (i.e. the user doesn't have to
// write `if_exists: Rename`), and not need a #[allow(non_camel_case_types)] flag, serde
// provides the option to modify the fields are deserialize/serialize time
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
    use std::borrow::Cow;

    #[test]
    fn rename_with_rename_conflict() -> Result<()> {
        let mut target = Cow::from(test_file_or_dir("test2.txt"));
        let expected = expected_path(&target, &Default::default());
        target.update(&ConflictOption::Rename, &Default::default()).unwrap();
        if target == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_overwrite_conflict() -> Result<()> {
        let mut target = Cow::from(test_file_or_dir("test2.txt"));
        let expected = target.clone();
        target.update(&ConflictOption::Overwrite, &Default::default()).unwrap();
        if target == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    #[should_panic] // unwrapping a None value
    fn rename_with_skip_conflict() {
        let mut target = Cow::from(test_file_or_dir("test2.txt"));
        target.update(&ConflictOption::Skip, &Default::default()).unwrap();
    }

    #[test]
    #[should_panic] // trying to modify a path that does not exist
    fn new_path_to_non_existing_file() {
        let mut target = Cow::from(test_file_or_dir("test_dir2").join("test1.txt"));
        #[cfg(debug_assertions)]
        debug_assert!(!target.exists());
        target.update(&ConflictOption::Rename, &Default::default()).unwrap();
    }
}
