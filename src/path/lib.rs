#[cfg(test)]
pub mod vars {
    use std::{
        env,
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
    };

    use crate::path::{
        get_stem_and_extension,
        Expandable,
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

    #[test]
    fn placeholder() -> Result<(), Error> {
        let tested = PathBuf::from("/home/cabero/Downloads/{parent.name}");
        let new_path = tested.expand_placeholders(&Path::new("/home/cabero/Documents/test.pdf"))?;
        let expected = PathBuf::from("/home/cabero/Downloads/Documents");
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Other))
        }
    }
}

#[cfg(test)]
mod filters {
    use crate::{
        path::MatchesFilters,
        user_config::rules::filters::Filters,
    };
    use std::{
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
    };

    #[test]
    fn test_temporary_files() -> Result<(), Error> {
        let crdownload = PathBuf::from("$HOME/Downloads/unsplash.jpg.crdownload");
        let tmp = PathBuf::from("$HOME/Downloads/unsplash.jpg.tmp");
        let part = PathBuf::from("$HOME/Downloads/unsplash.jpg.part");
        let download = PathBuf::from("$HOME/Downloads/unsplash.jpg.crdownload");
        let filters = Filters::default();
        for file in [crdownload, tmp, part, download].iter() {
            if file.matches_filters(&filters) {
                return Err(Error::new(ErrorKind::Other, "temporary file matched filters"));
            }
        }
        Ok(())
    }

    #[test]
    fn test_filters_extensions() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
        filters.extensions.push("pdf".to_string());
        if file.matches_filters(&filters) {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "file did not match the filters correctly".to_string(),
            ))
        }
    }
    #[test]
    fn test_filters_regex() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Documents/matricula.pdf");
        let filters = Filters::default();
        if file.matches_filters(&filters) {
            // the default regex is an empty one, so it should match everything
            // but we check for this possibility before trying to match
            Err(Error::new(ErrorKind::Other, "file matched an empty regex".to_string()))
        } else {
            Ok(())
        }
    }
    #[test]
    fn test_filters_filename_startswith() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
        filters.filename.startswith = "matricula".into();
        if file.matches_filters(&filters) {
            // the default regex is an empty one, so it should match everything
            // but we check for this possibility before trying to match
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "file did not match the filters correctly".to_string(),
            ))
        }
    }
    #[test]
    fn test_filters_filename_contains() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
        filters.filename.contains = "icula".into();
        if file.matches_filters(&filters) {
            // the default regex is an empty one, so it should match everything
            // but we check for this possibility before trying to match
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "file did not match the filters correctly".to_string(),
            ))
        }
    }
    #[test]
    fn test_filters_filename_endswith() -> Result<(), Error> {
        let file = PathBuf::from("/home/cabero/Documents/matricula.pdf");
        let mut filters = Filters::default();
        filters.filename.contains = "ula.pdf".into();
        if file.matches_filters(&filters) {
            // the default regex is an empty one, so it should match everything
            // but we check for this possibility before trying to match
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "file did not match the filters correctly".to_string(),
            ))
        }
    }
}
