use crate::{
    cli::Cli,
    user_config::rules::rule::Rule,
    PROJECT_NAME,
};
use clap::load_yaml;
use dirs::home_dir;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    io::{
        Error,
        ErrorKind,
    },
    path::{
        Path,
        PathBuf,
    },
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
    pub fn new(cli: &Cli) -> Result<Self, Error> {
        let path = UserConfig::path(cli);

        if !path.exists() {
            Self::create(&path)?;
        }

        let content = fs::read_to_string(&path)?;
        let config = serde_yaml::from_str(&content).expect("could not parse config file");

        Ok(config)
    }

    pub fn create(path: &Path) -> Result<(), Error> {
        // safe unwrap, dir is created at $HOME or $UserProfile%,
        // so it exists and the user must have permissions
        if path.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!(
                    "{} already exists in this directory",
                    path.file_name().unwrap().to_str().unwrap()
                ),
            ));
        }
        match path.parent() {
            Some(parent) => {
                if !parent.exists() {
                    std::fs::create_dir_all(path.parent().unwrap())?;
                }
                let config = load_yaml!("../../examples/config.yml");
                let mut output = String::new();
                let mut emitter = YamlEmitter::new(&mut output);
                emitter.dump(config).expect("ERROR: could not create starter config");
                std::fs::write(path, output)?;
            }
            None => panic!("home directory's parent folder should be defined"),
        }
        Ok(())
    }

    pub fn path(cli: &Cli) -> PathBuf {
        match cli.args.value_of("with_config") {
            Some(path) => PathBuf::from(path).canonicalize().expect("invalid path"),
            None => Self::default_path(),
        }
    }

    pub fn dir() -> PathBuf {
        home_dir()
            .expect("ERROR: cannot determine home directory")
            .join(format!(".{}", PROJECT_NAME))
    }

    pub fn default_path() -> PathBuf {
        Self::dir().join("config.yml")
    }

    /// returns a hashmap where the keys are paths and the values are tuples of rules
    /// and indices, which indicate the index of the key's corresponding folder in the rule's folders' list
    /// (i.e. the key is the ith folder in the corresponding rule)
    pub fn to_map(&self) -> HashMap<&PathBuf, Vec<(&Rule, usize)>> {
        let mut map = HashMap::new();
        for rule in self.rules.iter() {
            for (i, folder) in rule.folders.iter().enumerate() {
                if !map.contains_key(&folder.path) {
                    map.insert(&folder.path, Vec::new());
                }
                map.get_mut(&folder.path).unwrap().push((rule, i));
            }
        }
        map
    }
}
