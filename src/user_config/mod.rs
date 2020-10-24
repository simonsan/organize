use crate::{path::Expandable, user_config::rules::rule::Rule, ARGS};
use clap::{crate_name, load_yaml};
use dirs::home_dir;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use yaml_rust::YamlEmitter;

pub mod rules;

/// Represents the user's configuration file
/// ### Fields
/// * `path`: the path the user's config, either the default one or some other passed with the --with-config argument
/// * `rules`: a list of parsed rules defined by the user
#[derive(Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub rules: Vec<Rule>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl UserConfig {
    /// Creates a new UserConfig instance.
    /// It parses the configuration file
    /// and fills missing fields with either the defaults, in the case of global options,
    /// or with the global options, in the case of folder-level options.
    /// If the config file does not exist, it is created.
    /// ### Errors
    /// This constructor fails in the following cases:
    /// - The configuration file does not exist
    pub fn new() -> Self {
        let path = UserConfig::path();

        if !path.exists() {
            Self::create(&path);
        }

        let content = fs::read_to_string(&path).unwrap();
        let mut config: UserConfig = serde_yaml::from_str(&content).expect("could not parse config file");
        config.path = path;
        for (i, rule) in config.rules.iter().enumerate() {
            let action = &rule.actions;
            if action.r#move.is_some() && action.rename.is_some() {
                panic!(
                    "error: tried declaring both a `move` and `rename` action, which are incompatible (rule no. {})",
                    i
                )
            }
        }
        config
    }

    pub fn create(path: &Path) {
        debug_assert!(!path.exists());
        // safe unwrap, dir is created at $HOME or $UserProfile%,
        // so it exists and the user must have permissions
        match path.parent() {
            Some(parent) => {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)
                        .unwrap_or_else(|_| panic!("error: could not create config directory ({})", parent.display()));
                }
                let config = load_yaml!("../../examples/config.yml");
                let mut output = String::new();
                let mut emitter = YamlEmitter::new(&mut output);
                emitter.dump(config).expect("error: example config contains problems");
                std::fs::write(path, output)
                    .unwrap_or_else(|_| panic!("error: could not create config file ({})", path.display()));
            }
            None => panic!("config file's parent folder should be defined"),
        }
    }

    pub fn path() -> PathBuf {
        match ARGS.value_of("config") {
            Some(path) => PathBuf::from(path).expand_user().expand_vars().canonicalize().unwrap(),
            None => Self::default_path(),
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn dir() -> PathBuf {
        let dir = home_dir()
            .expect("ERROR: cannot determine home directory")
            .join(".config")
            .join(crate_name!());
        if !dir.exists() {
            fs::create_dir_all(&dir).expect("error: could not create config directory (permission denied)");
        }
        dir
    }

    pub fn default_path() -> PathBuf {
        Self::dir().join("config.yml")
    }

    /// returns a hashmap where the keys are paths and the values are tuples of rules
    /// and indices, which indicate the index of the key's corresponding folder in the rule's folders' list
    /// (i.e. the key is the ith folder in the corresponding rule)
    pub fn to_map(&self) -> HashMap<&Path, Vec<(&Rule, usize)>> {
        let mut map = HashMap::new();
        for rule in self.rules.iter() {
            for (i, folder) in rule.folders.iter().enumerate() {
                if !map.contains_key(folder.path.as_path()) {
                    map.insert(folder.path.as_path(), Vec::new());
                }
                map.get_mut(folder.path.as_path()).unwrap().push((rule, i));
            }
        }
        map
    }
}
