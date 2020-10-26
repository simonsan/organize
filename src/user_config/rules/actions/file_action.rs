use crate::{
    path::{Expandable, Update},
    string::Placeholder,
    subcommands::logs::{Level, Logger},
    user_config::rules::{
        actions::{Action, ConflictOption, Sep},
        deserialize::deserialize_path,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fs,
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
    result,
    str::FromStr,
};

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
    pub(super) fn helper(
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
