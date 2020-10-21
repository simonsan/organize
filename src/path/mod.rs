use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    subcommands::run::resolve_conflict,
    user_config::rules::{
        actions::{ConflictOption, Sep},
        filters::{Filename, Filters},
    },
    WATCHING,
};
use std::sync::atomic::Ordering;

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
    fn update(&self, if_exists: &ConflictOption, sep: &Sep) -> Option<PathBuf>;
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
    fn update(&self, if_exists: &ConflictOption, sep: &Sep) -> Option<Self> {
        debug_assert!(self.exists());

        match if_exists {
            ConflictOption::Skip => None,
            ConflictOption::Overwrite => Some(self.clone()),
            ConflictOption::Rename => {
                let (stem, extension) = get_stem_and_extension(&self);
                let mut new_path = self.clone();
                let mut n = 1;
                while new_path.exists() {
                    let new_filename = format!("{}{}({:?}).{}", stem, sep.as_str(), n, extension);
                    new_path.set_file_name(new_filename);
                    n += 1;
                }
                Some(new_path)
            }
            ConflictOption::Ask => {
                debug_assert_ne!(ConflictOption::default(), ConflictOption::Ask);
                let if_exists = if WATCHING.load(Ordering::SeqCst) {
                    Default::default()
                } else {
                    resolve_conflict(&self)
                };
                self.update(&if_exists, sep)
            }
        }
    }
}

pub trait Expandable {
    fn expand_user(&self) -> PathBuf;
    fn expand_vars(&self) -> PathBuf;
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
