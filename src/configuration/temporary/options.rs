use crate::{configuration, configuration::{
    options::Options,
    temporary::{
        rules::TemporaryRule,
        TemporaryConfigElement,
    },
}, utils};
use core::{
    default::Default,
    option::Option::{
        None,
        Some,
    },
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

    fn fill(&mut self, parent_rule: &TemporaryRule) -> Self {
        Self::default() + parent_rule.options.clone().unwrap_or_default() + self.clone()
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

impl Add for TemporaryOptions {
    type Output = Self;

    /// Performs the + operation.
    /// This addition is not commutative.
    /// The other object's fields are prioritized.
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            recursive: utils::combine_options(self.recursive, rhs.recursive, Some(false)),
            watch: utils::combine_options(self.watch, rhs.watch, Some(false)),
            system_files: utils::combine_options(self.system_files, rhs.system_files, Some(false)),
            ignore: utils::combine_options(self.ignore, rhs.ignore, None),
            suggestions: utils::combine_options(self.suggestions, rhs.suggestions, Some(false)),
            enabled: utils::combine_options(self.enabled, rhs.enabled, Some(true)),
            hidden_files: utils::combine_options(self.hidden_files, rhs.hidden_files, Some(true)),
        }
    }
}

impl Add for &TemporaryOptions {
    type Output = TemporaryOptions;

    fn add(self, rhs: Self) -> Self::Output {
        TemporaryOptions {
            recursive: utils::combine_options(self.recursive, rhs.recursive, Some(false)),
            watch: utils::combine_options(self.watch, rhs.watch, Some(false)),
            system_files: utils::combine_options(self.system_files, rhs.system_files, Some(false)),
            ignore: utils::combine_options(self.to_owned().ignore, rhs.to_owned().ignore, None),
            suggestions: utils::combine_options(self.suggestions, rhs.suggestions, Some(false)),
            enabled: utils::combine_options(self.enabled, rhs.enabled, Some(true)),
            hidden_files: utils::combine_options(self.hidden_files, rhs.hidden_files, Some(true)),
        }
    }
}
