mod cli;
mod config;
pub(crate) mod file;
mod logger;

#[cfg(test)]
pub mod tests {
    use crate::config::actions::ConflictOption;
    use crate::file::utils;
    use std::io::{Error, ErrorKind};
    use std::path::PathBuf;

    #[test]
    fn rename_with_rename_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test2.txt");
        let new_path = utils::new_path(&file1, &file2, ConflictOption::Rename)?;
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
        let new_path = utils::new_path(&file, &dir, ConflictOption::Rename)?;
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
        let new_path = utils::new_path(&file1, &file2, ConflictOption::Overwrite)?;
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
        let new_path = utils::new_path(&file, &dir, ConflictOption::Overwrite)?;
        let expected = PathBuf::from(format!("{}/test1.txt", dir.to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    #[should_panic(expected = "already exists")]
    fn rename_with_skip_conflict() {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test2.txt");
        utils::new_path(&file1, &file2, ConflictOption::Skip).unwrap();
    }

    #[test]
    #[should_panic(expected = "already exists")]
    fn move_with_skip_conflict() {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test_dir");
        utils::new_path(&file1, &dir, ConflictOption::Skip).unwrap();
    }
}
