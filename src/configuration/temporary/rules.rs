use crate::configuration::{
    rules::Rule,
    temporary::{
        actions::TemporaryActions,
        filters::TemporaryFilters,
        folders::TemporaryFolder,
        options::TemporaryOptions,
        TemporaryConfigElement,
    },
};
use core::{
    default::Default,
    option::Option::Some,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    io::{
        Error,
        ErrorKind,
    },
    path::Path,
    slice::Iter,
};

#[derive(Debug, Clone, Deserialize)]
pub struct TemporaryRule {
    pub actions: TemporaryActions,
    pub filters: TemporaryFilters,
    pub folders: Vec<TemporaryFolder>,
    pub options: Option<TemporaryOptions>,
}

impl TemporaryRule {
    pub fn unwrap(self) -> Rule {
        let mut folders = Vec::new();
        for folder in self.folders.iter() {
            folders.push(folder.clone().fill(&self).unwrap())
        }
        Rule {
            options: self.options.clone().unwrap_or_default().fill(&self).unwrap(),
            actions: self.actions.unwrap(),
            filters: self.filters.unwrap(),
            folders,
        }
    }
}

impl Default for TemporaryRule {
    fn default() -> Self {
        Self {
            actions: Default::default(),
            filters: Default::default(),
            folders: Default::default(),
            options: Some(Default::default()),
        }
    }
}

#[derive(Debug)]
pub struct TemporaryRules(Vec<TemporaryRule>);

impl TemporaryRules {
    /// Returns a new object containing the parsed rules from the user's config file.
    /// ### Errors
    /// This function will return an error in the following cases:
    /// - The config file does not contain a `rules` field
    /// - The path does not already exist.
    /// Other errors may also be returned according to OpenOptions::open.
    /// - It encounters while reading an error of a kind
    /// other than ErrorKind::Interrupted, or if the contents of the file are not valid UTF-8.
    pub fn new(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        let rules: HashMap<String, Vec<TemporaryRule>> =
            serde_yaml::from_str(&content).expect("could not parse config file");
        let mut rules = TemporaryRules(
            rules
                .get("rules")
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "ERROR: field 'rules' is missing"))
                .unwrap()
                .clone(),
        );
        rules.fill_missing_fields();
        Ok(rules)
    }

    /// Fills the missing fields of the user's config. Since most fields are optional,
    /// we need a safe way to ensure all needed fields are defined in the internal representation.
    ///
    /// We combine global options with default options, preserving (when possible) the global options.
    /// We then combine each folder's options with these modified global options, giving a higher
    /// priority to these folder-level options, since they're more specific.
    /// ### Return
    /// This function does not return anything. All mutations are done in place.
    pub fn fill_missing_fields(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        for rule in self.0.iter() {
            rules.push(rule.clone().unwrap())
        }
        rules
    }

    pub fn iter(&self) -> Iter<TemporaryRule> {
        self.0.iter()
    }

    pub fn validate(&self) {
        todo!()
    }
}
