use crate::configuration::actions::ConflictOption;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

fn new_dir(to: &Path) -> Result<PathBuf, Error> {
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
pub fn new_filepath(from: &Path, to: &Path, conflict_option: ConflictOption) -> Result<PathBuf, Error> {
    if to.exists() {
        return match conflict_option {
            ConflictOption::skip => Err(Error::new(
                ErrorKind::AlreadyExists,
                format!(
                    "ERROR: {} already exists",
                    to.to_str()
                        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "ERROR: cannot convert OsStr to &str"))?
                ),
            )),
            ConflictOption::rename => {
                let mut n = 1;

                let (stem, extension) = if to.is_file() {
                    get_stem_and_extension(to)?
                } else if to.is_dir() {
                    get_stem_and_extension(from)?
                } else {
                    panic!("file is neither a file nor a dir?")
                };
                let new_dir = new_dir(to)?;
                let mut new_path = to.to_path_buf();
                while new_path.exists() {
                    let new_filename = format!("{} ({:?}).{}", stem, n, extension);
                    new_path = new_dir.join(new_filename);
                    n += 1;
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
            ConflictOption::ask => todo!(),
        };
    }
    Ok(to.to_path_buf())
}

/// # Arguments
/// * `path`: A reference to a std::path::PathBuf
/// # Return
/// Returns the stem and extension of `path` if they exist and can be parsed, otherwise returns an Error
pub fn get_stem_and_extension(path: &Path) -> Result<(&str, &str), Error> {
    let stem = path
        .file_stem()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "file does not have a file stem (?)"))?
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to &str"))?;
    let extension = path
        .extension()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "file does not have an extension"))?
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "cannot convert OsStr to str"))?;

    Ok((stem, extension))
}
