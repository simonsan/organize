#[cfg(test)]
mod tests {
    use std::{
        env,
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
    };

    use crate::{
        path::Expandable,
        utils::testing::{
            project_dir,
            tests_dir,
        },
    };
    use dirs::home_dir;

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
        // TODO: remove var when finished
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
        assert!(
            env::var("PROJECT_DIR2").is_err(),
            "PROJECT_DIR should not be a valid environment variable for this test"
        );
        let tested = PathBuf::from("$PROJECT_DIR/tests");
        tested.expand_vars();
    }
}
