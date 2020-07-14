use crate::config::actions::ConflictOption;
use crate::file::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[cfg(test)]
#[test]
fn new_path_with_rename_conflict() -> Result<(), Error> {
    let file1 = File::from("/home/cabero/Code/Rust/d-organizer/src/tests/files/test1.txt")?;
    let file2 = File::from("/home/cabero/Code/Rust/d-organizer/src/tests/files/test2.txt")?;
    let new_path = file1.new_path(&file2.path, ConflictOption::Rename)?;
    let expected = PathBuf::from("/home/cabero/Code/Rust/d-organizer/src/tests/files/test2 (1).txt");
    if new_path == expected {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
    }
}

#[test]
fn new_path_with_overwrite_conflict() -> Result<(), Error> {
    let file1 = File::from("/home/cabero/Code/Rust/d-organizer/src/tests/files/test1.txt")?;
    let file2 = File::from("/home/cabero/Code/Rust/d-organizer/src/tests/files/test2.txt")?;
    let new_path = file1.new_path(&file2.path, ConflictOption::Overwrite)?;
    if new_path == file2.path {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "filepath after rename is not as expected"))
    }
}

#[test]
#[should_panic(expected = "already exists")]
fn new_path_with_skip_conflict() {
    let file1 = File::from("/home/cabero/Code/Rust/d-organizer/src/tests/files/test1.txt").unwrap();
    let file2 = File::from("/home/cabero/Code/Rust/d-organizer/src/tests/files/test2.txt").unwrap();
    file1.new_path(&file2.path, ConflictOption::Skip).unwrap();
}
