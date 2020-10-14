use std::{
    fs,
    io::{
        Error,
        ErrorKind,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cli::{
        Cli,
        config_path,
    },
    user_config::rules::rule::Rule,
    utils,
    utils::expand_env_vars,
};

/// Represents the user's configuration file
/// ### Fields
/// * `path`: the path the user's config, either the default one or some other passed with the --with-config argument
/// * `rules`: a list of parsed rules defined by the user
#[derive(Deserialize, Serialize, Clone, Debug)]
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
        let path = config_path(cli);

        if !path.exists() {
            utils::create_config_file(&path)?;
        }

        let content = fs::read_to_string(&path)?;
        let mut config: Self = serde_yaml::from_str(&content).expect("could not parse config file");

        for rule in config.rules.iter_mut() {
            for folder in rule.folders.iter_mut() {
                folder.path = expand_env_vars(&folder.path);
            }
        }

        Ok(config)
    }

    /// Validates the user's config.
    /// ### Errors
    /// This function returns an error in the following cases:
    /// - An empty string was provided as the path to a folder
    /// - The path supplied to a folder does not exist
    /// - The path supplied to a folder is not a directory
    /// - No path was supplied to a folder
    pub fn validate(self) -> Result<Self, Error> {
        for (i, rule) in self.rules.iter().enumerate() {
            rule.actions.check_conflicting_actions()?;
            for (j, folder) in rule.folders.iter().enumerate() {
                if folder.path.display().to_string().eq("") {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "path defined in field 'path' cannot be an empty value (rule {}, folder {})",
                            j, i
                        ),
                    ));
                } else if !folder.path.exists() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("path defined in field 'path' does not exist (rule {}, folder {})", j, i),
                    ));
                } else if !folder.path.is_dir() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "path defined in field 'path' is not a directory (rule {}, folder {})",
                            j, i
                        ),
                    ));
                }
            }
        }
        Ok(self)
    }
}
