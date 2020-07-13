use crate::config::actions::utils::get_stem_and_extension;
use crate::config::actions::ConflictOption;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub struct File {
    path: PathBuf,
}

impl<'a> File {
    pub fn from(path: &'a mut PathBuf) -> Result<Self, Error> {
        assert!(
            path.is_file(),
            "ERROR: tried creating a config::actions::File from a non-file PathBuf"
        );
        assert!(
            path.parent().is_some(),
            "ERROR: file does not exist within a valid directory"
        );
        match path.file_name() {
            Some(_) => Ok(File {
                path: path.to_path_buf(),
            }),
            None => Err(Error::new(ErrorKind::InvalidInput, "ERROR: invalid path")),
        }
    }

    pub fn copy(&self) -> Result<&Self, Error> {
        Ok(self)
    }

    fn new_dir(&self, to: &Path) -> Result<PathBuf, Error> {
        Ok(match to.is_file() {
            true => to
                .parent()
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        "ERROR: invalid parent directory for target location",
                    )
                })?
                .to_path_buf(),
            false => to.to_path_buf(),
        })
    }

    fn new_path(&self, to: &Path, conflict_option: ConflictOption) -> Result<PathBuf, Error> {
        if to.exists() {
            return match conflict_option {
                ConflictOption::Skip => Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!(
                        "ERROR: {} already exists",
                        to.to_str()
                            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "ERROR: cannot convert OsStr to &str"))?
                    ),
                )),
                ConflictOption::Rename => {
                    let mut n = 1;
                    let (stem, extension) = get_stem_and_extension(&self.path)?;
                    let new_dir = self.new_dir(to)?;
                    let mut new_path = to.to_path_buf();
                    while new_path.exists() && conflict_option == ConflictOption::Rename {
                        let new_filename = format!("{} ({:?}).{}", stem, n, extension);
                        new_path = new_dir.join(new_filename);
                        n += 1;
                    }
                    Ok(new_path)
                }
                ConflictOption::Overwrite => Ok(to.to_path_buf()),
            };
        }
        Ok(to.to_path_buf())
    }

    pub fn r#move(&mut self, dst: &PathBuf, conflict_option: ConflictOption) -> Result<&Self, Error> {
        assert!(dst.is_dir());
        let new_path = self.new_path(stem, extension, dst, conflict_option)?;
        std::fs::rename(self.path.as_path(), dst.as_path())?;
        self.path = new_path;
        Ok(self)
    }

    pub fn rename(&mut self, dst: &PathBuf, conflict_option: ConflictOption) -> Result<&Self, Error> {
        assert!(dst.is_file());
        let (stem, extension) = get_stem_and_extension(dst)?;
        let new_path = self.new_path(stem, extension, dst, conflict_option)?;
        std::fs::rename(self.path.as_path(), dst.as_path())?;
        self.path = new_path;
        Ok(self)
    }
}
