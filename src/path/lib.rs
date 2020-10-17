#[cfg(test)]
pub mod tests {
    use std::{
        env,
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
    };

    use crate::{
        file::get_stem_and_extension,
        path::Expandable,
    };
    use dirs::home_dir;
    use std::path::Path;

    pub fn project_dir() -> PathBuf {
        // 'cargo test' must be run from the project directory, where Cargo.toml is
        // even if you run it from some other folder inside the project
        // 'cargo test' will move to the project root
        env::current_dir().unwrap()
    }

    pub fn tests_dir() -> PathBuf {
        project_dir().join("tests")
    }

    pub fn test_file_or_dir(filename: &str) -> PathBuf {
        tests_dir().join("files").join(filename)
    }

    pub fn expected_path(file: &Path, sep: &str) -> PathBuf {
        let (stem, extension) = get_stem_and_extension(file);
        let parent = file.parent().unwrap();
        parent.join(format!("{}{}(1).{}", stem, sep, extension))
    }

    #[test]
    fn home() -> Result<(), Error> {
        let tested = PathBuf::from("$HOME/Documents");
        let expected = home_dir().unwrap().join("Documents");
        if tested.expand_vars() == expected {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "the environment variable wasn't properly expanded",
            ))
        }
    }

    #[test]
    fn new_var() -> Result<(), Error> {
        env::set_var("PROJECT_DIR", project_dir());
        let tested = PathBuf::from("$PROJECT_DIR/tests");
        if tested.expand_vars() == tests_dir() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "the environment variable wasn't properly expanded",
            ))
        }
    }

    #[test]
    #[should_panic]
    fn non_existing_var() {
        let var = "PROJECT_DIR_2";
        assert!(
            env::var(var).is_err(),
            "PROJECT_DIR should not be a valid environment variable for this test"
        );
        let tested = PathBuf::from(format!("${}/tests", var));
        tested.expand_vars();
    }
}
