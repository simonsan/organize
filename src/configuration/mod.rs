pub mod actions;
pub mod conflicts;
pub mod filters;
pub mod folders;
pub mod options;

use crate::{
    configuration::{
        actions::Actions,
        filters::Filters,
        folders::Folder,
        options::Options,
    },
    subcommands::config::Rules,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::PathBuf,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub actions: Actions,
    pub filters: Option<Filters>,
    pub folders: Vec<Folder>,
    pub options: Option<Options>,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            actions: Default::default(),
            filters: None,
            folders: Default::default(),
            options: Some(Default::default()),
        }
    }
}

/// returns a hashmap where the keys are paths and the values are tuples of rules
/// and indices that indicate the index of the key's corresponding folder in the rule's folders' list
pub fn folder2rules(rules: &Rules) -> HashMap<&PathBuf, Vec<(&Rule, usize)>> {
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
