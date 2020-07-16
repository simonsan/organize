pub mod utils;

use crate::cli::Cli;
use crate::configuration::options::Options;
use crate::configuration::Rule;
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents the user's configuration file
/// ### Fields
/// * `path`: the path the user's config, either the default one or some other passed with the --with-config argument
/// * `rules`: a list of parsed rules defined by the user
pub struct UserConfig {
    pub path: PathBuf,
    pub rules: Rules,
}

pub struct Rules(Vec<Rule>);

impl Rules {
    /// Returns a new object containing the parsed rules from the user's config file.
    /// ### Errors
    /// This function will return an error in the following cases:
    /// - The config file does not contain a `rules` field
    /// - The path does not already exist.
    /// Other errors may also be returned according to OpenOptions::open.
    /// - It encounters while reading an error of a kind
    /// other than ErrorKind::Interrupted, or if the contents of the file are not valid UTF-8.
    pub(in crate::subcommands::config) fn new(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        let rules: HashMap<String, Vec<Rule>> = serde_yaml::from_str(&content).expect("could not parse config file");
        let mut rules = Rules(
            rules
                .get("rules")
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "field 'rules' is missing"))
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
    pub(in crate::subcommands::config) fn fill_missing_fields(&mut self) {
        // since we defined addition between Options objects as not commutative
        // if we want to preserve the most specific object's fields we need to place
        // it to the right of the + operator
        let default_options = &Options::default();
        for rule in self.0.iter_mut() {
            rule.options = Some(default_options + rule.options.as_ref().unwrap_or_else(|| default_options));
            for folder in rule.folders.iter_mut() {
                match &folder.options {
                    Some(options) => folder.options = Some(rule.options.as_ref().unwrap() + options),
                    None => folder.options = rule.options.clone(),
                }
            }
        }
    }
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
    pub fn new(args: &Cli) -> Result<Self, Error> {
        let path = match args.subcommand.1.value_of("with_config") {
            Some(path) => PathBuf::from(path),
            None => dirs::home_dir()
                .expect("ERROR: cannot determine home directory")
                .join(".d-organizer")
                .join("config.yml"),
        };

        if !path.exists() {
            utils::create_config_file(&path)?;
        }

        let rules = Rules::new(&path)?;

        Ok(UserConfig {
            path,
            rules,
        })
    }

    /// Launches an editor to modify the default config.
    /// This function represents the `config` subcommand without any arguments.
    /// ### Errors
    /// This functions returns an error in the following cases:
    /// - There is no $EDITOR environment variable.
    /// ### Panics
    /// This functions panics in the following cases:
    /// - The $EDITOR env. variable was found but its process could not be started.
    pub fn edit(&self) -> Result<&Self, Error> {
        match std::env::var("EDITOR") {
            Ok(editor) => {
                let mut editor = Command::new(editor);
                editor
                    .arg(self.path.display().to_string())
                    .spawn()
                    .expect("ERROR: failed to run editor")
                    .wait()
                    .expect("ERROR: command was not running");
                Ok(self)
            }
            Err(_) => {
                let error_msg = utils::prompt_editor_env_var();
                Err(Error::new(ErrorKind::NotFound, error_msg))
            }
        }
    }

    /// Validates the user's config.
    /// ### Errors
    /// This function returns an error in the following cases:
    /// - An empty string was provided as the path to a folder
    /// - The path supplied to a folder does not exist
    /// - The path supplied to a folder is not a directory
    /// - No path was supplied to a folder
    pub fn validate(self) -> Result<Self, Error> {
        for (i, rule) in self.rules.0.iter().enumerate() {
            for (j, folder) in rule.folders.iter().enumerate() {
                match &folder.path {
                    Some(path) => {
                        if path.display().to_string().eq("") {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "path defined in field 'path' cannot be an empty value (rule {}, folder {})",
                                    j, i
                                ),
                            ));
                        } else if !path.exists() {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("path defined in field 'path' does not exist (rule {}, folder {})", j, i),
                            ));
                        } else if !path.is_dir() {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "path defined in field 'path' is not a directory (rule {}, folder {})",
                                    j, i
                                ),
                            ));
                        }
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "field 'path' is required but it was not supplied (rule {}, folder {})",
                                j, i
                            ),
                        ))
                    }
                }
            }
        }
        Ok(self)
    }
}
