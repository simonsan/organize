pub mod copy;
pub mod delete;
pub mod echo;
pub mod r#move;
pub mod python;
pub mod rename;
pub mod shell;

use crate::{
    path::{Expandable, Update},
    string::Placeholder,
    subcommands::logs::{Level, Logger},
    user_config::{
        rules::{
            actions::{
                copy::Copy, delete::Delete, echo::Echo, python::Python, r#move::Move, rename::Rename, shell::Shell,
            },
            deserialize::deserialize_path,
        },
        UserConfig,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fs,
    io::{Error, ErrorKind, Result},
    ops::Deref,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    result,
    str::FromStr,
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
    Echo,
    Trash,
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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Script(String);

impl Deref for Script {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Script {
    pub(super) fn write(content: &str, path: &Path) -> Result<PathBuf> {
        let dir = UserConfig::dir().join("scripts");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let script = dir.join("temp_script");
        let content = content.expand_placeholders(path)?;
        fs::write(&script, content)?;
        Ok(script)
    }

    pub(super) fn run(&self, path: &Path, program: &str) -> Result<()> {
        let content = self.deref();
        let script = Script::write(content, path)?;
        let output = Command::new(program)
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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Actions {
    pub echo: Option<Echo>,
    pub shell: Option<Shell>,
    pub python: Option<Python>,
    pub delete: Option<Delete>,
    pub copy: Option<Copy>,
    pub r#move: Option<Move>,
    pub rename: Option<Rename>,
}

impl Actions {
    pub fn run(&self, path: PathBuf) {
        let mut path = Cow::from(path);
        if let Some(echo) = &self.echo {
            if let Err(e) = echo.run(&path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if let Some(python) = &self.python {
            if let Err(e) = python.run(&path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if let Some(shell) = &self.shell {
            if let Err(e) = shell.run(&path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if let Some(copy) = &self.copy {
            if let Err(e) = copy.run(&mut path) {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if self.r#move.is_some() ^ self.rename.is_some() {
            let result = if let Some(r#move) = &self.r#move {
                r#move.run(&mut path)
            } else {
                self.rename.as_ref().unwrap().run(&mut path)
            };
            if let Err(e) = result {
                eprintln!("error: {}", e.to_string());
                return;
            }
        }
        if let Some(delete) = &self.delete {
            if *delete.deref() {
                if let Err(e) = delete.run(&path) {
                    eprintln!("error: {}", e.to_string());
                    return;
                }
            }
        }
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

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub struct FileAction {
    #[serde(deserialize_with = "deserialize_path")]
    pub to: PathBuf,
    #[serde(default)]
    pub if_exists: ConflictOption,
    #[serde(default)]
    pub sep: Sep,
}

impl From<PathBuf> for FileAction {
    fn from(path: PathBuf) -> Self {
        Self {
            to: path.expand_user().expand_vars(),
            if_exists: Default::default(),
            sep: Default::default(),
        }
    }
}

impl FromStr for FileAction {
    type Err = ();

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let path = s.parse::<PathBuf>().unwrap();
        Ok(Self::from(path))
    }
}

impl FileAction {
    /// Helper function for the move, rename and copy actions.
    /// # Args:
    /// - `path`: a reference to a Cow smart pointer containing the original file
    /// - `to`: the destination path of `path`
    /// - `if_exists`: variable that helps resolve any naming conflicts between `path` and `to`
    /// - `sep`: counter separator (e.g. in "file (1).test", `sep` would be a whitespace; in "file-(1).test", it would be a hyphen)
    /// - `type`: whether this helper should move, rename or copy the given file (`path`)
    pub(in crate::user_config::rules::actions) fn helper(
        path: &mut Cow<Path>,
        to: &Path,
        if_exists: &ConflictOption,
        sep: &Sep,
        r#type: Action,
    ) -> Result<()> {
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
            return Ok(());
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
