use crate::config::actions::ConflictOption;
use crate::utils::get_stem_and_extension;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
pub struct File {
    pub path: PathBuf,
}

impl<'a> File {
    pub fn from(s: &str) -> Result<Self, Error> {
        let path = canonicalize(s)?;
        if !path.exists() || !path.is_file() || !path.parent().is_some() {
            return Err(Error::from(ErrorKind::InvalidInput));
        }
        Ok(File {
            path,
        })
    }

    pub fn copy(&self, to: &Path, conflict_option: ConflictOption) -> Result<PathBuf, Error> {
        let new_path = self.new_path(to, conflict_option)?;
        std::fs::copy(self.path.as_path(), new_path.as_path()).expect("cannot write file (permission denied)");
        Ok(new_path)
    }

    fn new_dir(&self, to: &Path) -> Result<PathBuf, Error> {
        Ok(if to.is_file() {
            to.parent()
                .ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        "ERROR: invalid parent directory for target location",
                    )
                })?
                .to_path_buf()
        } else if to.is_dir() {
            to.to_path_buf()
        } else {
            panic!("path is neither a file nor a directory")
        })
    }

    pub fn new_path(&self, to: &Path, conflict_option: ConflictOption) -> Result<PathBuf, Error> {
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
                    let (stem, extension) = get_stem_and_extension(to)?;
                    let new_dir = self.new_dir(to)?;
                    let mut new_path = to.to_path_buf();
                    while new_path.exists() {
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

    // works for move too
    pub fn rename(&self, to: &Path, conflict_option: ConflictOption) -> Result<PathBuf, Error> {
        let new_path = self.new_path(&to, conflict_option)?;
        std::fs::rename(self.path.as_path(), new_path.as_path()).expect("couldn't rename file");
        Ok(new_path)
    }

    pub fn delete(&self) -> Result<(), Error> {
        std::fs::remove_file(&self.path)
    }
}
