mod cli;
mod configuration;
pub(crate) mod file;
mod subcommands;
pub use colored::*;

#[cfg(test)]
pub mod tests {
    use super::configuration::conflict_option::ConflictOption;
    use super::configuration::options::Options;
    use super::file::utils;
    use std::io::{Error, ErrorKind};
    use std::path::PathBuf;

    #[test]
    fn rename_with_rename_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test2.txt");
        let new_path = utils::new_filepath(&file1, &file2, ConflictOption::Rename)?;
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
        let new_path = utils::new_filepath(&file, &dir, ConflictOption::Rename)?;
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
        let new_path = utils::new_filepath(&file1, &file2, ConflictOption::Overwrite)?;
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
        let new_path = utils::new_filepath(&file, &dir, ConflictOption::Overwrite)?;
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
        utils::new_filepath(&file1, &file2, ConflictOption::Skip).unwrap();
    }

    #[test]
    #[should_panic(expected = "already exists")]
    fn move_with_skip_conflict() {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/d-organizer/tests/files/test_dir");
        utils::new_filepath(&file1, &dir, ConflictOption::Skip).unwrap();
    }

    #[test]
    fn combine_options() -> Result<(), Error> {
        let opt1 = Options {
            recursive: None,
            watch: None,
            ignore: None,
            suggestions: None,
            enabled: None,
            system_files: None,
        };
        let opt2 = Options::default();
        let result = opt1.to_owned() + opt2.to_owned();
        if result != opt2 {
            eprintln!("{:?}, {:?}", opt1, opt2);
            return Err(Error::from(ErrorKind::Other));
        }
        let opt1 = Options {
            recursive: None,
            watch: Some(true),
            ignore: Some(vec![PathBuf::from("/home/cabero/Downloads/ignored_dir")]),
            suggestions: None,
            enabled: None,
            system_files: None,
        };
        let opt2 = Options {
            recursive: None,
            watch: Some(false),
            ignore: None,
            suggestions: None,
            enabled: None,
            system_files: None,
        };
        let expected = Options {
            recursive: Some(false),
            watch: Some(false),
            ignore: Some(vec![PathBuf::from("/home/cabero/Downloads/ignored_dir")]),
            suggestions: Some(false),
            enabled: Some(true),
            system_files: Some(false),
        };
        if opt1.to_owned() + opt2.to_owned() == expected {
            Ok(())
        } else {
            eprintln!("{:?}, {:?}", opt1, opt2);
            Err(Error::from(ErrorKind::Other))
        }
    }
}
