use crate::configuration::{
    TemporaryConfigElement,
    TemporaryRule,
};
use serde::Deserialize;
use std::{
    ops::Add,
    path::PathBuf,
};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
pub struct TemporaryOptions {
    pub recursive: Option<bool>,
    pub watch: Option<bool>,
    pub ignore: Option<Vec<PathBuf>>,
    pub suggestions: Option<bool>,
    pub enabled: Option<bool>,
    pub system_files: Option<bool>,
    pub hidden_files: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub recursive: bool,
    pub watch: bool,
    pub ignore: Vec<PathBuf>,
    pub suggestions: bool,
    pub enabled: bool,
    pub system_files: bool,
    pub hidden_files: bool,
}

impl TemporaryConfigElement<Options> for TemporaryOptions {
    fn unwrap(self) -> Options {
        Options {
            recursive: self.recursive.unwrap(),
            watch: self.watch.unwrap(),
            ignore: self.ignore.unwrap(),
            suggestions: self.suggestions.unwrap(),
            enabled: self.enabled.unwrap(),
            system_files: self.system_files.unwrap(),
            hidden_files: self.hidden_files.unwrap(),
        }
    }

    fn fill(self, parent_rule: &TemporaryRule) -> Self {
        Self::default() + parent_rule.options.clone().unwrap_or_default() + self
    }
}

impl Default for TemporaryOptions {
    fn default() -> Self {
        TemporaryOptions {
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

pub fn combine_options<T>(lhs: Option<T>, rhs: Option<T>, default: Option<T>) -> Option<T> {
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

impl Add for TemporaryOptions {
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

impl Add for &TemporaryOptions {
    type Output = TemporaryOptions;

    fn add(self, rhs: Self) -> Self::Output {
        TemporaryOptions {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{
        Error,
        ErrorKind,
    };

    #[test]
    fn none_plus_default() -> Result<(), Error> {
        let left = TemporaryOptions {
            recursive: None,
            watch: None,
            ignore: None,
            suggestions: None,
            enabled: None,
            system_files: None,
            hidden_files: None,
        };
        let right = TemporaryOptions::default();
        let result = left.to_owned() + right.to_owned();
        if result == right {
            Ok(())
        } else {
            eprintln!("{:?}, {:?}", left, right);
            Err(Error::from(ErrorKind::Other))
        }
    }

    #[test]
    fn random_combine() -> Result<(), Error> {
        let left = TemporaryOptions {
            recursive: None,
            watch: Some(true),
            ignore: Some(vec![PathBuf::from("/home/cabero/Downloads/ignored_dir")]),
            suggestions: None,
            enabled: None,
            system_files: None,
            hidden_files: Some(false),
        };
        let right = TemporaryOptions {
            recursive: None,
            watch: Some(false),
            ignore: None,
            suggestions: None,
            enabled: None,
            system_files: None,
            hidden_files: Some(true),
        };
        let expected = TemporaryOptions {
            recursive: Some(false),
            watch: Some(false),
            ignore: Some(vec![PathBuf::from("/home/cabero/Downloads/ignored_dir")]),
            suggestions: Some(false),
            enabled: Some(true),
            system_files: Some(false),
            hidden_files: Some(true),
        };

        if left.to_owned() + right.to_owned() == expected {
            Ok(())
        } else {
            eprintln!("{:?}, {:?}", left, right);
            Err(Error::from(ErrorKind::Other))
        }
    }
}
