use crate::{
    configuration::temporary::{
        actions,
        actions::new_filepath,
        conflicts::{
            ConflictOption,
            ConflictingActions,
            ConflictingFileOperation,
        },
    },
    file::File,
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

#[derive(Debug, Clone)]
pub struct Actions {
    pub echo: Option<String>,
    pub shell: Option<String>,
    pub trash: Option<bool>,
    pub delete: Option<bool>,
    pub copy: Option<ConflictingFileOperation>,
    pub r#move: Option<ConflictingFileOperation>,
    pub rename: Option<ConflictingFileOperation>,
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
        // should check that it's some before calling this method
        if self.copy.as_ref().unwrap().if_exists == ConflictOption::skip || from == self.copy.as_ref().unwrap().to {
            return Ok(());
        }

        let new_path = actions::new_filepath(
            from,
            &self.copy.as_ref().unwrap().to,
            &self.copy.as_ref().unwrap().if_exists,
            watching,
        )?;
        println!("{}", new_path.display());
        std::fs::copy(from, new_path.as_path()).expect("cannot write file (permission denied)");
        Ok(())
    }

    fn rename(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        // should check that it's some before calling this method
        if self.rename.as_ref().unwrap().if_exists == ConflictOption::skip || from == self.rename.as_ref().unwrap().to {
            return Ok(from.to_path_buf());
        }
        let dst = actions::new_filepath(
            from,
            &self.copy.as_ref().unwrap().to,
            &self.copy.as_ref().unwrap().if_exists,
            watching,
        )?;
        std::fs::rename(from, dst.as_path()).expect("couldn't rename file");
        Ok(dst)
    }

    fn r#move(&self, from: &Path, watching: bool) -> Result<PathBuf, Error> {
        // should check that it's some before calling this method
        if self.r#move.as_ref().unwrap().if_exists == ConflictOption::skip || from == self.r#move.as_ref().unwrap().to {
            return Ok(from.to_path_buf());
        }
        if !self.r#move.as_ref().unwrap().to.exists() {
            fs::create_dir_all(&self.r#move.as_ref().unwrap().to)?;
        }
        let dst = new_filepath(
            from,
            &self.r#move.as_ref().unwrap().to.join(from.file_name().unwrap()),
            &self.r#move.as_ref().unwrap().if_exists,
            watching,
        )?;

        std::fs::rename(from, dst.as_path()).expect("couldn't rename file");
        Ok(dst)
    }

    fn delete(&self, path: &Path) -> Result<(), Error> {
        std::fs::remove_file(path)
    }
}
