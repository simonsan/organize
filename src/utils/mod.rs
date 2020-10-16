use std::{
    collections::HashMap,
    path::PathBuf,
};

use crate::user_config::rules::rule::Rule;

/// returns a hashmap where the keys are paths and the values are tuples of rules
/// and indices that indicate the index of the key's corresponding folder in the rule's folders' list
pub fn path2rules(rules: &[Rule]) -> HashMap<&PathBuf, Vec<(&Rule, usize)>> {
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

#[cfg(test)]
pub fn project_dir() -> PathBuf {
    use std::env;
    // 'cargo test' must be run from the project directory, where Cargo.toml is
    // even if you run it from some other folder inside the project
    // 'cargo test' will move to the project root
    env::current_dir().unwrap()
}

#[cfg(test)]
pub fn tests_dir() -> PathBuf {
    project_dir().join("tests")
}

#[cfg(test)]
pub fn test_file_or_dir(filename: &str) -> PathBuf {
    tests_dir().join("files").join(filename)
}
