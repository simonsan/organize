use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

use crate::{
    commands::run::resolve_conflict,
    file::get_stem_and_extension,
    user_config::rules::actions::ConflictOption,
};

mod lib;

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
    fn fullpath(&self) -> PathBuf;
    fn expand_user(&self) -> PathBuf;
    fn expand_vars(&self) -> PathBuf;
    // fn expand_placeholders(&self, path: &Path) -> PathBuf;
}

impl Expandable for PathBuf {
    fn fullpath(&self) -> Self {
        // we're not making up the filepath, so running canonicalize should not fail
        let mut path = self.expand_user().expand_vars();
        if path.exists() {
            path = path.canonicalize().unwrap();
        }
        path
    }

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

    // fn expand_placeholders(&self, path: &Path) -> Self {
    //     let as_str = self.to_str().unwrap().to_string();
    // }
}
