pub mod utils;

use crate::cli::Cli;
use crate::configuration::options::Options;
use crate::configuration::Rule;
use crate::subcommands::SubCommands;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use yaml_rust::YamlLoader;

pub struct UserConfig {
    pub path: PathBuf,
    pub rules: Vec<Rule>,
}

impl UserConfig {
    pub fn new(args: &Cli) -> Result<Self, Error> {
        let mut config = UserConfig {
            path: PathBuf::new(),
            rules: Vec::new(),
        };
        config.path = config.path(args);
        config.rules = config.parse()?;
        config.fill_missing_fields();

        if !config.path.exists() {
            utils::create_config_file(&config.path)?;
        };
        Ok(config)
    }

    fn parse(&self) -> Result<Vec<Rule>, Error> {
        let content = fs::read_to_string(&self.path)?;
        let rules: HashMap<String, Vec<Rule>> = serde_yaml::from_str(&content).expect("could not parse config file");
        let rules = rules.get("rules").unwrap().clone();
        Ok(rules)
    }

    fn fill_missing_fields(&mut self) {
        let default_options = &Options::default();
        for rule in self.rules.iter_mut() {
            rule.options = Some(default_options + rule.options.as_ref().unwrap_or_else(|| default_options));
            for folder in rule.folders.iter_mut() {
                match &folder.options {
                    Some(options) => folder.options = Some(rule.options.as_ref().unwrap() + options),
                    None => folder.options = rule.options.clone(),
                }
            }
        }
    }

    fn path(&self, args: &Cli) -> PathBuf {
        if (args.subcommand.0 == SubCommands::Run || args.subcommand.0 == SubCommands::Watch)
            && args.subcommand.1.is_present("with_config")
        {
            PathBuf::from(args.subcommand.1.value_of("with_config").unwrap())
        } else {
            dirs::home_dir()
                .expect("ERROR: cannot determine home directory")
                .join(".d-organizer")
                .join("config.yml")
        }
    }

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

    pub fn validate(self) -> Result<Self, Error> {
        for (i, rule) in self.rules.iter().enumerate() {
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
