use crate::configuration::rules::Rule;
use std::{
    collections::HashMap,
    path::PathBuf,
};

pub mod actions;
pub mod conflicts;
pub mod filters;
pub mod folders;
pub mod options;
pub mod rules;
pub mod temporary;

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

pub fn combine_options<T>(lhs: Option<T>, rhs: Option<T>, default: Option<T>) -> Option<T> {
    if lhs.is_some() && rhs.is_none() {
        lhs
    } else if lhs.is_none() && rhs.is_some() {
        rhs
    } else if lhs.is_none() && rhs.is_none() {
        default
    } else {
        // both are some
        rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::temporary::options::TemporaryOptions;
    use std::{
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
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
