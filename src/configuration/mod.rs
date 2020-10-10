pub mod actions;
pub mod conflicts;
pub mod filters;
pub mod folders;
pub mod options;

use crate::{
    configuration::{
        actions::Actions,
        filters::{
            Filters,
            TemporaryFilters,
        },
        folders::{
            Folder,
            TemporaryFolder,
        },
        options::{
            Options,
            TemporaryOptions,
        },
    },
    subcommands::edit::TemporaryRules,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::PathBuf,
};

trait TemporaryConfigElement<T> {
    fn unwrap(self) -> T;
    fn fill(self, parent_rule: &TemporaryRule) -> Self;
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemporaryRule {
    pub actions: Actions,
    pub filters: TemporaryFilters,
    pub folders: Vec<TemporaryFolder>,
    pub options: Option<TemporaryOptions>,
}

impl TemporaryRule {
    pub fn unwrap(&self) -> Rule {
        let mut folders = Vec::new();
        for folder in self.folders.iter() {
            folders.push(folder.clone().fill(self).unwrap())
        }
        Rule {
            actions: self.actions.clone(),
            filters: self.filters.clone().unwrap(),
            folders,
            options: self.options.clone().unwrap_or_default().fill(self).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub actions: Actions,
    pub filters: Filters,
    pub folders: Vec<Folder>,
    pub options: Options,
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

/// returns a hashmap where the keys are paths and the values are tuples of rules
/// and indices that indicate the index of the key's corresponding folder in the rule's folders' list
pub fn folder2rules(rules: &[Rule]) -> HashMap<&PathBuf, Vec<(&Rule, usize)>> {
    let mut map = HashMap::new();
    for rule in rules.iter() {
        for (i, folder) in rule.folders.iter().enumerate() {
            if !map.contains_key(&folder.path) {
                map.insert(&folder.path, Vec::new());
            }
            map.get_mut(&folder.path).unwrap().push((rule, i));
        }
    }
    map
}
