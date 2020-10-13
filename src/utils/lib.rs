#[cfg(test)]
mod new_filepath {
    use crate::{
        configuration::conflicts::ConflictOption,
        utils::new_filepath,
    };
    use std::{
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
    };

    static WATCHING: bool = false;
    #[test]
    fn rename_with_rename_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test2.txt");
        let new_path = new_filepath(&file1, &file2, &ConflictOption::Rename, WATCHING)?;
        let expected = PathBuf::from(format!("{}/test2 (1).txt", file2.parent().unwrap().to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn move_with_rename_conflict() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test_dir");
        let new_path = new_filepath(
            &file,
            &dir.join(file.file_name().unwrap()),
            &ConflictOption::Rename,
            WATCHING,
        )?;
        let expected = PathBuf::from(format!("{}/test1 (1).txt", dir.to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_overwrite_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test2.txt");
        let new_path = new_filepath(&file1, &file2, &ConflictOption::Overwrite, WATCHING)?;
        if new_path == file2 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }
    #[test]
    fn move_with_overwrite_conflict() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test_dir");
        let new_path = new_filepath(
            &file,
            &dir.join(file.file_name().unwrap()),
            &ConflictOption::Overwrite,
            WATCHING,
        )?;
        let expected = PathBuf::from(format!("{}/test1.txt", dir.to_str().unwrap()));
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn rename_with_skip_conflict() -> Result<(), Error> {
        let file1 = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test1.txt");
        let file2 = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test2.txt");
        let expected = new_filepath(&file1, &file2, &ConflictOption::Skip, false).unwrap();
        if file1 == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
        }
    }

    #[test]
    fn move_with_skip_conflict() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test1.txt");
        let dir = PathBuf::from("/home/cabero/Code/Rust/organize/tests/files/test_dir");
        let expected = new_filepath(
            &file,
            &dir.join(file.file_name().unwrap()),
            &ConflictOption::Skip,
            false,
        )
        .unwrap();
        if file == expected {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "filepath after move is not as expected"))
        }
    }
}
// #[cfg(test)]
// mod combine_options {
//     use std::{
//         io::{
//             Error,
//             ErrorKind,
//         },
//         path::PathBuf,
//     };
//
//     #[test]
//     fn none_plus_default() -> Result<(), Error> {
//         let left = TemporaryOptions {
//             recursive: None,
//             watch: None,
//             ignore: None,
//             suggestions: None,
//             enabled: None,
//             system_files: None,
//             hidden_files: None,
//         };
//         let right = TemporaryOptions::default();
//         let result = left.to_owned() + right.to_owned();
//         if result == right {
//             Ok(())
//         } else {
//             eprintln!("{:?}, {:?}", left, right);
//             Err(Error::from(ErrorKind::Other))
//         }
//     }
//
//     #[test]
//     fn random_combine() -> Result<(), Error> {
//         let left = TemporaryOptions {
//             recursive: None,
//             watch: Some(true),
//             ignore: Some(vec![PathBuf::from("/home/cabero/Downloads/ignored_dir")]),
//             suggestions: None,
//             enabled: None,
//             system_files: None,
//             hidden_files: Some(false),
//         };
//         let right = TemporaryOptions {
//             recursive: None,
//             watch: Some(false),
//             ignore: None,
//             suggestions: None,
//             enabled: None,
//             system_files: None,
//             hidden_files: Some(true),
//         };
//         let expected = TemporaryOptions {
//             recursive: Some(false),
//             watch: Some(false),
//             ignore: Some(vec![PathBuf::from("/home/cabero/Downloads/ignored_dir")]),
//             suggestions: Some(false),
//             enabled: Some(true),
//             system_files: Some(false),
//             hidden_files: Some(true),
//         };
//
//         if left.to_owned() + right.to_owned() == expected {
//             Ok(())
//         } else {
//             eprintln!("{:?}, {:?}", left, right);
//             Err(Error::from(ErrorKind::Other))
//         }
//     }
// }
