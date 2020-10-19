use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    string::Capitalize,
    subcommands::run::resolve_conflict,
    user_config::rules::{
        actions::ConflictOption,
        filters::{Filename, Filters},
    },
};
use regex::Regex;
use std::io::{Error, ErrorKind};

pub mod lib;

pub trait MatchesFilters {
    fn matches_filters(&self, filters: &Filters) -> bool;
    fn is_hidden(&self) -> bool;
}

impl MatchesFilters for PathBuf {
    fn matches_filters(&self, filters: &Filters) -> bool {
        let extension = self.extension().unwrap_or_default().to_str().unwrap_or_default();
        let temporary_file_extensions = ["crdownload", "part", "tmp", "download"];
        if !extension.is_empty() && temporary_file_extensions.contains(&extension) {
            return false;
        }

        let as_str = self.to_str().unwrap();
        if !filters.regex.to_string().is_empty() && filters.regex.is_match(&as_str) {
            return true;
        }

        let Filename {
            startswith,
            endswith,
            contains,
            ..
        } = &filters.filename;

        let filename = self.file_name().unwrap().to_str().unwrap();
        if !startswith.is_empty() && filename.starts_with(startswith) {
            return true;
        }
        if !endswith.is_empty() && filename.ends_with(endswith) {
            return true;
        }
        if !contains.is_empty() && filename.contains(contains) {
            return true;
        }
        if !filters.extensions.is_empty() && filters.extensions.contains(&extension.to_string()) {
            return true;
        }
        false
    }

    fn is_hidden(&self) -> bool {
        self.file_name().unwrap().to_str().unwrap().starts_with('.')
    }
}

pub trait Update {
    fn update(&self, if_exists: &ConflictOption, sep: &str, watching: bool) -> Option<PathBuf>;
}

impl Update for PathBuf {
    ///  When trying to rename a file to a path that already exists, calling update() on the
    ///  target path will return a new valid path.
    ///  # Args
    /// * `if_exists`: option to resolve the naming conflict
    /// * `sep`: if `if_exists` is set to rename, `sep` will go between the filename and the added counter
    /// * `is_watching`: whether this function is being run from a watcher or not
    /// # Return
    /// This function will return `Some(new_path)` if `if_exists` is not set to skip, otherwise it returns `None`
    fn update(&self, if_exists: &ConflictOption, sep: &str, watching: bool) -> Option<Self> {
        #[cfg(debug_assertions)]
        assert!(self.exists());

        match if_exists {
            ConflictOption::Skip => None,
            ConflictOption::Overwrite => Some(self.clone()),
            ConflictOption::Rename => {
                let (stem, extension) = get_stem_and_extension(&self);
                let mut new_path = self.clone();
                let mut n = 1;
                while new_path.exists() {
                    let new_filename = format!("{}{}({:?}).{}", stem, sep, n, extension);
                    new_path.set_file_name(new_filename);
                    n += 1;
                }
                Some(new_path)
            }
            ConflictOption::Ask => {
                assert_ne!(ConflictOption::default(), ConflictOption::Ask);
                let if_exists = if watching {
                    Default::default()
                } else {
                    resolve_conflict(&self)
                };
                self.update(&if_exists, sep, watching)
            }
        }
    }
}

pub trait Expandable {
    fn expand_user(&self) -> PathBuf;
    fn expand_vars(&self) -> PathBuf;
    fn expand_placeholders(&self, path: &Path) -> Result<PathBuf, Error>;
}

impl Expandable for PathBuf {
    fn expand_user(&self) -> Self {
        let str = self.to_str().unwrap().to_string();
        Self::from(str.replace("~", "$HOME"))
    }

    fn expand_vars(&self) -> Self {
        self.components()
            .map(|component| {
                let component: &Path = component.as_ref();
                let component = component.to_str().unwrap();
                if component.starts_with('$') {
                    env::var(component.replace('$', ""))
                        .unwrap_or_else(|_| panic!("error: environment variable '{}' could not be found", component))
                } else {
                    component.to_string()
                }
            })
            .collect()
    }

    fn expand_placeholders(&self, path: &Path) -> Result<Self, Error> {
        let mut as_str = self.to_str().unwrap().to_string();
        let regex = Regex::new(r"\{\w+(?:\.\w+)*}").unwrap();
        for span in regex.find_iter(self.to_str().unwrap()) {
            let placeholders = span
                .as_str()
                .trim_matches(|x| x == '{' || x == '}')
                .split('.')
                .collect::<Vec<_>>();
            let mut current_value = path.to_path_buf();
            for placeholder in placeholders {
                current_value = match placeholder {
                    "path" => current_value,
                    "parent" => current_value
                        .parent()
                        .ok_or_else(|| {
                            Error::new(
                                ErrorKind::Other,
                                format!(
                                    "modified path has no {} (original filepath: {})",
                                    placeholder,
                                    path.display()
                                ),
                            )
                        })?
                        .into(),
                    "name" => current_value
                        .file_name()
                        .ok_or_else(|| {
                            Error::new(
                                ErrorKind::Other,
                                format!(
                                    "modified path has no {} (original filepath: {})",
                                    placeholder,
                                    path.display()
                                ),
                            )
                        })?
                        .into(),
                    "stem" => current_value
                        .file_stem()
                        .ok_or_else(|| {
                            Error::new(
                                ErrorKind::Other,
                                format!(
                                    "modified path has no {} (original filepath: {})",
                                    placeholder,
                                    path.display()
                                ),
                            )
                        })?
                        .into(),
                    "extension" => current_value
                        .extension()
                        .ok_or_else(|| {
                            Error::new(
                                ErrorKind::Other,
                                format!(
                                    "modified path has no {} (original filepath: {})",
                                    placeholder,
                                    path.display()
                                ),
                            )
                        })?
                        .into(),
                    "to_uppercase" => current_value.to_str().unwrap().to_uppercase().into(),
                    "to_lowercase" => current_value.to_str().unwrap().to_lowercase().into(),
                    "capitalize" => current_value.to_str().unwrap().to_string().capitalize().into(),
                    _ => panic!("unknown placeholder"),
                }
            }
            as_str = as_str
                .replace(&span.as_str(), current_value.to_str().unwrap())
                .replace("//", "/");
        }
        Ok(as_str.into())
    }
}

/// # Arguments
/// * `path`: A reference to a std::path::PathBuf
/// # Return
/// Returns the stem and extension of `path` if they exist and can be parsed, otherwise returns an Error
fn get_stem_and_extension(path: &Path) -> (&str, &str) {
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap_or_default().to_str().unwrap();

    (stem, extension)
}
