#[cfg(test)]
mod new_path {
    use std::{
        env,
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
    };

    use crate::user_config::rules::actions::{
        ConflictOption,
        ConflictingFileOperation,
    };

    fn project_dir() -> PathBuf {
        // 'cargo test' must be run from the project directory, where Cargo.toml is
        // even if you run it from some other folder inside the project
        // 'cargo test' will move to the project root
        env::current_dir().unwrap()
    }

    fn tests_directory() -> PathBuf {
        project_dir().join("tests")
    }

    fn test_file_or_dir(filename: &str) -> PathBuf {
        tests_directory().join("files").join(filename)
    }
    static WATCHING: bool = false;

    #[test]
    fn rename_with_rename_conflict() -> Result<(), Error> {
        let action = ConflictingFileOperation::from(test_file_or_dir("test2.txt"));
        let file = test_file_or_dir("test1.txt");
        let new_path = action.get_new_path(&file, WATCHING)?;
        let expected = PathBuf::from(format!(
            "{}/test2 (1).txt",
            action.to.parent().unwrap().to_str().unwrap()
        ));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    #[should_panic]
    fn new_path_in_non_existing_dir() {
        let mut action = ConflictingFileOperation::from(test_file_or_dir("test_dir2"));
        action.if_exists = ConflictOption::Overwrite;
        let file = test_file_or_dir("test1.txt");
        action.get_new_path(&file, WATCHING).unwrap();
    }

    #[test]
    #[should_panic]
    fn new_path_from_non_existing_path() {
        let mut action = ConflictingFileOperation::from(test_file_or_dir("test_dir2"));
        action.if_exists = ConflictOption::Overwrite;
        let file = test_file_or_dir("test10.txt");
        assert!(!file.exists());
        action.get_new_path(&file, WATCHING).unwrap();
    }

    #[test]
    fn move_with_rename_conflict() -> Result<(), Error> {
        let action = ConflictingFileOperation::from(test_file_or_dir("test_dir"));
        let file = test_file_or_dir("test1.txt");
        let new_path = action.get_new_path(&file, WATCHING)?;
        let expected = PathBuf::from(format!("{}/test1 (1).txt", action.to.to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_overwrite_conflict() -> Result<(), Error> {
        let mut action = ConflictingFileOperation::from(test_file_or_dir("test2.txt"));
        action.if_exists = ConflictOption::Overwrite;
        let file = test_file_or_dir("test1.txt");
        let new_path = action.get_new_path(&file, WATCHING)?;
        if new_path == action.to {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }
    #[test]
    fn move_with_overwrite_conflict() -> Result<(), Error> {
        let mut action = ConflictingFileOperation::from(test_file_or_dir("test_dir"));
        action.if_exists = ConflictOption::Overwrite;
        let file = test_file_or_dir("test1.txt");
        let new_path = action.get_new_path(&file, WATCHING)?;
        let expected = PathBuf::from(format!("{}/test1.txt", action.to.to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_skip_conflict() -> Result<(), Error> {
        let mut action = ConflictingFileOperation::from(test_file_or_dir("test2.txt"));
        action.if_exists = ConflictOption::Skip;
        let original = test_file_or_dir("test1.txt");
        let new = action.get_new_path(&original, WATCHING);
        if new.is_err() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn move_with_skip_conflict() -> Result<(), Error> {
        let mut action = ConflictingFileOperation::from(test_file_or_dir("test_dir"));
        action.if_exists = ConflictOption::Skip;
        let original = test_file_or_dir("test1.txt");
        let new = action.get_new_path(&original, WATCHING);
        if new.is_err() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after move is not as expected"))
        }
    }
}
