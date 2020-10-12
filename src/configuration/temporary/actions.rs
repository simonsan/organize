use crate::{
    configuration::{
        actions::Actions,
        temporary::{
            conflicts::{
                ConflictOption,
                ConflictingActions,
                ConflictingFileOperation,
                TemporaryConflictingFileOperation,
            },
            folders::expand_env_vars,
        },
    },
    file::{
        get_stem_and_extension,
        File,
    },
};
use colored::Colorize;
use serde::Deserialize;
use std::{
    borrow::BorrowMut,
    fs::create_dir_all,
    io,
    io::{
        Error,
        ErrorKind,
        Read,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct TemporaryActions {
    pub echo: Option<String>,
    pub shell: Option<String>,
    pub trash: Option<bool>,
    pub delete: Option<bool>,
    pub copy: Option<TemporaryConflictingFileOperation>,
    pub r#move: Option<TemporaryConflictingFileOperation>,
    pub rename: Option<TemporaryConflictingFileOperation>,
}

impl Default for TemporaryActions {
    fn default() -> Self {
        TemporaryActions {
            echo: Some("".to_string()),
            shell: Some("".to_string()),
            trash: Some(false),
            delete: Some(false),
            copy: Some(Default::default()),
            r#move: Some(Default::default()),
            rename: Some(Default::default()),
        }
    }
}

impl TemporaryActions {
    fn unwrap_action(&self, action: &Option<TemporaryConflictingFileOperation>) -> Option<ConflictingFileOperation> {
        match action.clone() {
            Some(mut action) => {
                action.fill();
                Some(action.unwrap())
            }
            None => None,
        }
    }

    pub fn unwrap(self) -> Actions {
        Actions {
            r#move: self.unwrap_action(&self.r#move),
            copy: self.unwrap_action(&self.copy),
            rename: self.unwrap_action(&self.rename),
            echo: self.echo,
            shell: self.shell,
            trash: self.trash,
            delete: self.delete,
        }
    }
}
/// Helper function for the 'rename' and 'move' actions.
/// It computes the appropriate new path for the file wanting to be renamed or moved.
/// In case of a name conflict, it will decide what new path to return based on a resolver parameter
/// to avoid unwanted overwrites.
/// # Args
/// * `from`: path representing the original file's path
/// * `to`: path representing the target path (can be a file or a directory)
/// * `conflict_option`: configuration option that helps adapt the new path
/// # Errors
/// This function will return an error in the following case:
/// * The target path exists and `conflict_option` is set to skip
pub fn new_filepath(from: &Path, to: &Path, conflict_option: &ConflictOption) -> Result<PathBuf, Error> {
    if to.exists() {
        return match conflict_option {
            ConflictOption::skip => Ok(from.to_path_buf()),
            ConflictOption::rename => {
                let (stem, extension) = get_stem_and_extension(to.to_path_buf())?;
                let new_dir = to.parent().unwrap();
                let mut new_path = to.to_path_buf();

                if new_path.exists() {
                    let mut n = 1;
                    while new_path.exists() {
                        let new_filename = format!("{} ({:?}).{}", stem, n, extension);
                        new_path = new_dir.join(new_filename);
                        n += 1;
                    }
                }
                Ok(new_path)
            }
            ConflictOption::overwrite => {
                if to.is_file() {
                    Ok(to.to_path_buf())
                } else if to.is_dir() {
                    Ok(to.join(from.file_name().unwrap()))
                } else {
                    panic!("file is neither a file nor a dir?")
                }
            }
            ConflictOption::ask => {
                let input = resolve_name_conflict(to)?;
                new_filepath(from, to, &input)
            }
        };
    }
    Ok(to.to_path_buf())
}

impl TemporaryActions {}

pub fn resolve_name_conflict(dst: &Path) -> Result<ConflictOption, Error> {
    print!(
        "A file named {} already exists in the destination.\n [(o)verwrite / (r)ename / (s)kip]: ",
        dst.file_name().unwrap().to_str().unwrap().underline().bold()
    );
    io::stdout().flush().unwrap();

    let mut buf = [0; 1];
    io::stdin().read_exact(&mut buf).unwrap();
    let buf = buf[0];

    if buf == 111 {
        Ok(ConflictOption::overwrite)
    } else if buf == 114 {
        Ok(ConflictOption::rename)
    } else if buf == 115 {
        Ok(ConflictOption::skip)
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "ERROR: invalid option"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_with_rename_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test2.txt");
        let new_path = new_filepath(&file1, &file2, &ConflictOption::rename)?;
        let expected = PathBuf::from(format!("{}/test2 (1).txt", file2.parent().unwrap().to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn move_with_rename_conflict() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test_dir");
        let new_path = new_filepath(&file, &dir.join(file.file_name().unwrap()), &ConflictOption::rename)?;
        let expected = PathBuf::from(format!("{}/test1 (1).txt", dir.to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_overwrite_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test2.txt");
        let new_path = new_filepath(&file1, &file2, &ConflictOption::overwrite)?;
        if new_path == file2 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }
    #[test]
    fn move_with_overwrite_conflict() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test_dir");
        let new_path = new_filepath(&file, &dir.join(file.file_name().unwrap()), &ConflictOption::overwrite)?;
        let expected = PathBuf::from(format!("{}/test1.txt", dir.to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_skip_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test2.txt");
        let expected = new_filepath(&file1, &file2, &ConflictOption::skip).unwrap();
        if file1 == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn move_with_skip_conflict() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test_dir");
        let expected = new_filepath(&file, &dir.join(file.file_name().unwrap()), &ConflictOption::skip).unwrap();
        if file == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after move is not as expected"))
        }
    }
}
