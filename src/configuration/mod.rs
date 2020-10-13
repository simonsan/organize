use crate::configuration::{
    folders::Folder,
    rules::Rule,
};
use std::{
    collections::HashMap,
    path::PathBuf,
};
use clap::ArgMatches;
use std::io::{Error, ErrorKind};
use crate::config_path;
use crate::configuration::temporary::rules::TemporaryRules;

pub mod actions;
pub mod filters;
pub mod folders;
pub mod options;
pub mod rules;
pub mod temporary;

/// Represents the user's configuration file
/// ### Fields
/// * `path`: the path the user's config, either the default one or some other passed with the --with-config argument
/// * `rules`: a list of parsed rules defined by the user
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
    pub fn new(args: &ArgMatches) -> Result<Self, Error> {
        let path = match args.value_of("with_config") {
            Some(path) => PathBuf::from(path),
            None => config_path(),
        };

        if !path.exists() {
            crate::utils::create_config_file(&path)?;
        }

        let temp_rules = TemporaryRules::new(&path)?;
        let mut rules = Vec::new();
        for temp_rule in temp_rules.0 {
            rules.push(temp_rule.unwrap())
        }

        Ok(UserConfig {
            rules,
        })
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
