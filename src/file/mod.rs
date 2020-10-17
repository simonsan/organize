mod lib;

use crate::{
    path::Expandable,
    user_config::rules::filters::{
        Filename,
        Filters,
    },
};
use std::path::{
    Path,
    PathBuf,
};

// TODO: Integrate the File functionality into PathBuf

pub struct File {
    pub filename: String,
    pub stem: String,
    pub extension: String,
    pub path: PathBuf,
    pub is_hidden: bool,
}

impl From<PathBuf> for File {
    fn from(path: PathBuf) -> Self {
        let (stem, extension) = get_stem_and_extension(&path);
        let filename = path.file_name().unwrap().to_str().unwrap();
        Self {
            is_hidden: filename.starts_with('.'),
            filename: filename.to_string(),
            stem: stem.to_string(),
            extension: extension.to_string(),
            path: path.fullpath(),
        }
    }
}

impl From<&Path> for File {
    fn from(path: &Path) -> Self {
        let (stem, extension) = get_stem_and_extension(path);
        let filename = path.file_name().unwrap().to_str().unwrap();
        Self {
            is_hidden: filename.starts_with('.'),
            filename: filename.to_string(),
            stem: stem.to_string(),
            extension: extension.to_string(),
            path: path.to_path_buf().fullpath(),
        }
    }
}

impl From<&str> for File {
    fn from(path: &str) -> Self {
        let path = PathBuf::from(path);
        Self::from(path)
    }
}

impl File {
    pub fn matches_filters(&self, filters: &Filters) -> bool {
        // TODO test this function
        let temporary_file_extensions = ["crdownload", "part", "tmp", "download"];
        if temporary_file_extensions.contains(&self.extension.as_str()) {
            return false;
        }

        let path = self.path.to_str().unwrap();
        if !filters.regex.to_string().is_empty() && filters.regex.is_match(&path) {
            return true;
        }
        let Filename {
            startswith,
            endswith,
            contains,
            ..
        } = &filters.filename;
        if !startswith.is_empty() && self.filename.starts_with(startswith) {
            return true;
        }
        if !endswith.is_empty() && self.filename.ends_with(endswith) {
            return true;
        }
        if !contains.is_empty() && self.filename.contains(contains) {
            return true;
        }
        if !filters.extensions.is_empty() && filters.extensions.contains(&self.extension) {
            return true;
        }
        false
    }
}

/// # Arguments
/// * `path`: A reference to a std::path::PathBuf
/// # Return
/// Returns the stem and extension of `path` if they exist and can be parsed, otherwise returns an Error
pub fn get_stem_and_extension(path: &Path) -> (&str, &str) {
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap_or_default().to_str().unwrap();

    (stem, extension)
}
