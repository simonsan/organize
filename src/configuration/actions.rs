use crate::{
    configuration::conflicts::{
        ConflictOption,
        ConflictingActions,
        ConflictingFileOperation,
    },
    file::File,
    utils::new_filepath,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fs,
    io::{
        Error,
        ErrorKind,
    },
    path::{
        Path,
        PathBuf,
    },
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Actions {
    pub echo: Option<String>,
    pub shell: Option<String>,
    pub trash: Option<bool>,
    pub delete: Option<bool>,
    pub copy: Option<ConflictingFileOperation>,
    pub r#move: Option<ConflictingFileOperation>,
    pub rename: Option<ConflictingFileOperation>,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            echo: None,
            shell: None,
            trash: None,
            delete: None,
            copy: None,
            rename: None,
            r#move: None,
        }
    }
}

impl Actions {
    pub fn run(&self, file: &mut File, watching: bool) -> Result<(), Error> {
        if self.copy.is_some() {
            self.copy(&file.path, watching)?;
        }

        // TODO the following three are conflicting operations - validate this
        if self.r#move.is_some() {
            file.path = self.r#move(&file.path, watching)?;
        }

        if self.rename.is_some() {
            file.path = self.rename(&file.path, watching)?;
        }
        if self.delete.is_some() {
            self.delete(&file.path)?;
        }
        Ok(())
    }

    pub fn check_conflicting_actions(&self) -> Result<ConflictingActions, Error> {
        let mut conflicting_options = Vec::new();
        if self.r#move.is_some() {
            conflicting_options.push(ConflictingActions::Move);
        }
        if self.rename.is_some() {
            conflicting_options.push(ConflictingActions::Rename);
        }
        if self.delete.is_some() {
            conflicting_options.push(ConflictingActions::Delete);
        }
        if conflicting_options.len() == 1 {
            Ok(conflicting_options.get(0).unwrap().clone())
        } else if conflicting_options.is_empty() {
            Ok(ConflictingActions::None)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "too many conflicting actions"))
        }
    }

    fn copy(&self, from: &Path, watching: bool) -> Result<(), Error> {
        assert!(self.copy.is_some()); // should check that it's some before calling this method
        let copy = self.copy.as_ref().unwrap();
        if copy.if_exists == ConflictOption::Skip || from == copy.to {
            return Ok(());
        }

        let dst = new_filepath(from, &copy.to, &copy.if_exists, watching)?;
        std::fs::copy(from, dst.as_path()).expect("cannot write file (permission denied)");
        Ok(())
    }

    fn rename(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(self.rename.is_some()); // should check that it's some before calling this method
        let rename = self.rename.as_ref().unwrap();

        if rename.if_exists == ConflictOption::Skip || from == rename.to {
            return Ok(from.to_path_buf());
        }
        let dst = new_filepath(from, &rename.to, &rename.if_exists, watching)?;
        std::fs::rename(from, &dst).expect("couldn't rename file");
        Ok(dst)
    }

    fn r#move(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        assert!(self.r#move.is_some()); // should check that it's some before calling this method
        let r#move = self.r#move.as_ref().unwrap();

        if r#move.if_exists == ConflictOption::Skip || from == r#move.to {
            return Ok(from.to_path_buf());
        }
        if !r#move.to.exists() {
            fs::create_dir_all(&r#move.to)?;
        }

        let dst = new_filepath(
            from,
            &r#move.to.join(from.file_name().unwrap()),
            &r#move.if_exists,
            watching,
        )?;

        std::fs::rename(from, dst.as_path()).expect("couldn't rename file");
        Ok(dst)
    }

    fn delete(&self, path: &Path) -> Result<(), Error> {
        std::fs::remove_file(path)
    }
}
