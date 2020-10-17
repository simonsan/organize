#[cfg(test)]
mod new_path {
    use std::io::{
        Error,
        ErrorKind,
    };

    use crate::{
        path::{
            lib::vars::{
                expected_path,
                test_file_or_dir,
            },
            Update,
        },
        user_config::rules::actions::ConflictOption,
    };

    static WATCHING: bool = false;
    static SEP: &str = " ";

    #[test]
    fn rename_with_rename_conflict() -> Result<(), Error> {
        let target = test_file_or_dir("test2.txt");
        let expected = expected_path(&target, SEP);
        let new_path = target.update(&ConflictOption::Rename, SEP, WATCHING).unwrap();
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }
    #[test]
    fn rename_with_overwrite_conflict() -> Result<(), Error> {
        let target = test_file_or_dir("test2.txt");
        let expected = target.clone();
        let new_path = target.update(&ConflictOption::Overwrite, SEP, WATCHING).unwrap();
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }
    #[test]
    #[should_panic] // unwrapping a None value
    fn rename_with_skip_conflict() {
        let target = test_file_or_dir("test2.txt");
        target.update(&ConflictOption::Skip, SEP, WATCHING).unwrap();
    }

    #[test]
    #[should_panic] // trying to modify a path that does not exist
    fn new_path_to_non_existing_file() {
        let target = test_file_or_dir("test_dir2").join("test1.txt");
        assert!(!target.exists());
        target.update(&ConflictOption::Rename, SEP, WATCHING).unwrap();
    }
}
