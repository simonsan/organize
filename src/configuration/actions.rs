use crate::{
    configuration::{
        conflicts::{
            ConflictOption,
            ConflictingFileOperation,
        },
        options::combine_options,
    },
    file::{
        get_stem_and_extension,
        File,
    },
};
use colored::Colorize;
use serde::Deserialize;
use std::{
    fs::create_dir_all,
    io,
    io::{
        Error,
        ErrorKind,
        Read,
        Write,
    },
    ops::Add,
    path::{
        Path,
        PathBuf,
    },
};

#[derive(PartialEq, Debug, Clone, Deserialize)]
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
        Actions {
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

impl Add for Actions {
    type Output = Self;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The right-hand object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        Actions {
            rename: combine_options(self.rename, rhs.rename, Some(Default::default())),
            r#move: combine_options(self.r#move, rhs.r#move, Some(Default::default())),
            copy: combine_options(self.copy, rhs.copy, Some(Default::default())),
            delete: combine_options(self.delete, rhs.delete, Some(false)),
            trash: combine_options(self.trash, rhs.trash, Some(false)),
            shell: combine_options(self.shell, rhs.shell, Some("".to_string())),
            echo: combine_options(self.echo, rhs.echo, Some("".to_string())),
        }
    }
}

impl Add for &Actions {
    type Output = Actions;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The right-hand object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        let r#move = self.r#move.clone().unwrap_or_default() + rhs.r#move.clone().unwrap_or_default();
        let copy = self.copy.clone().unwrap_or_default() + rhs.copy.clone().unwrap_or_default();
        let rename = self.rename.clone().unwrap_or_default() + rhs.rename.clone().unwrap_or_default();
        Actions {
            rename: Some(rename),
            r#move: Some(r#move),
            copy: Some(copy),
            delete: combine_options(self.delete, rhs.delete, Some(false)),
            trash: combine_options(self.trash, rhs.trash, Some(false)),
            shell: combine_options(self.clone().shell, rhs.clone().shell, Some("".to_string())),
            echo: combine_options(self.clone().echo, rhs.clone().echo, Some("".to_string())),
        }
    }
}

pub fn process_actions(actions: &Actions, file: &mut File, i: &usize) -> Result<(), Error> {
    if let Some(action) = &actions.copy {
        let dst = &action.to;
        copy(
            &file.path,
            dst,
            action.if_exists.as_ref().unwrap_or_else(|| &ConflictOption::rename),
        )?;
    }

    // TODO the following three are conflicting operations - validate this
    if let Some(action) = &actions.r#move {
        let dst = &action.to;
        file.path = r#move(
            &file.path,
            dst,
            action.if_exists.as_ref().unwrap_or_else(|| &ConflictOption::rename),
        )?;
    };
    if let Some(action) = &actions.rename {
        let dst = &action.to;
        file.path = rename(
            &file.path,
            dst,
            action.if_exists.as_ref().unwrap_or_else(|| &ConflictOption::rename),
        )?;
    }
    if actions.delete.is_some() {
        delete(&file.path)?;
    }
    Ok(())
}

fn copy(from: &Path, to: &Path, conflict_option: &ConflictOption) -> Result<(), Error> {
    if conflict_option == &ConflictOption::skip || from == to {
        return Ok(());
    }
    let new_path = new_filepath(from, to, conflict_option)?;
    std::fs::copy(from, new_path.as_path()).expect("cannot write file (permission denied)");
    Ok(())
}

fn rename(from: &Path, to: &Path, conflict_option: &ConflictOption) -> Result<PathBuf, Error> {
    if conflict_option == &ConflictOption::skip || from == to {
        return Ok(from.to_path_buf());
    }
    let dst = new_filepath(from, &to, conflict_option)?;
    std::fs::rename(from, dst.as_path()).expect("couldn't rename file");
    Ok(dst)
}

fn r#move(from: &Path, to: &Path, conflict_option: &ConflictOption) -> Result<PathBuf, Error> {
    if conflict_option == &ConflictOption::skip || from == to {
        return Ok(from.to_path_buf());
    }
    if !to.exists() {
        create_dir_all(to.to_str().unwrap())?;
    }
    let dst = to.join(from.file_name().unwrap());
    rename(from, &dst, conflict_option)?;
    Ok(dst)
}

fn delete(path: &Path) -> Result<(), Error> {
    std::fs::remove_file(path)
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
                let (stem, extension) = get_stem_and_extension(to)?;
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
