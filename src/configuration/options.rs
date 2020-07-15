use serde::Deserialize;
use std::ops::Add;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Options {
    pub recursive: Option<bool>,
    pub watch: Option<bool>,
    pub ignore: Option<Vec<PathBuf>>,
    pub suggestions: Option<bool>,
    pub enabled: Option<bool>,
    pub system_files: Option<bool>,
    pub hidden_files: Option<bool>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            recursive: Some(false),
            watch: Some(false),
            ignore: Some(Vec::new()),
            suggestions: Some(false),
            enabled: Some(true),
            system_files: Some(false),
            hidden_files: Some(false),
        }
    }
}

fn combine_options<T>(lhs: Option<T>, rhs: Option<T>, default: Option<T>) -> Option<T> {
    if lhs.is_some() && rhs.is_none() {
        lhs
    } else if lhs.is_none() && rhs.is_some() {
        rhs
    } else if lhs.is_none() && rhs.is_none() {
        default
    } else {
        rhs
    }
}

impl Add for Options {
    type Output = Self;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The other object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            recursive: combine_options(self.recursive, rhs.recursive, Some(false)),
            watch: combine_options(self.watch, rhs.watch, Some(false)),
            system_files: combine_options(self.system_files, rhs.system_files, Some(false)),
            ignore: combine_options(self.ignore, rhs.ignore, None),
            suggestions: combine_options(self.suggestions, rhs.suggestions, Some(false)),
            enabled: combine_options(self.enabled, rhs.enabled, Some(true)),
            hidden_files: combine_options(self.hidden_files, rhs.hidden_files, Some(true)),
        }
    }
}

impl Add for &Options {
    type Output = Options;

    fn add(self, rhs: Self) -> Self::Output {
        Options {
            recursive: combine_options(self.recursive, rhs.recursive, Some(false)),
            watch: combine_options(self.watch, rhs.watch, Some(false)),
            system_files: combine_options(self.system_files, rhs.system_files, Some(false)),
            ignore: combine_options(self.to_owned().ignore, rhs.to_owned().ignore, None),
            suggestions: combine_options(self.suggestions, rhs.suggestions, Some(false)),
            enabled: combine_options(self.enabled, rhs.enabled, Some(true)),
            hidden_files: combine_options(self.hidden_files, rhs.hidden_files, Some(true)),
        }
    }
}
