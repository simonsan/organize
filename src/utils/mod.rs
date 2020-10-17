#[cfg(test)]
pub mod testing {
    use crate::file::get_stem_and_extension;
    use std::path::{
        Path,
        PathBuf,
    };

    pub fn project_dir() -> PathBuf {
        use std::env;
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
}
