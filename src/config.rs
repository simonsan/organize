use crate::cli::Cli;
use serde::Deserialize;
use std::io::{Error, ErrorKind};
use std::ops::BitXor;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub extensions: Option<Vec<String>>,
    pub suffix: Option<String>,
    pub prefix: Option<String>,
    pub regex: Option<String>,
    pub dst: String,
}

impl Rule {
    fn validate(&self) -> Result<(), Error> {
        if self.extensions.as_ref().unwrap().is_empty() || self.extensions.as_ref().unwrap().iter().any(|x| x == "") {
            return Err(Error::new(ErrorKind::InvalidData, "extensions cannot be empty or contain empty strings"))
        }
        if !self.has_valid_detailed_rules() {
            return Err(Error::new(ErrorKind::InvalidData, "suffix/prefix options are mutually exclusive with regex"))
        }

        let new_path = PathBuf::from(self.dst.as_str());
        if !new_path.exists() {
            return Err(Error::new(ErrorKind::InvalidData, "invalid dst field, please provide a valid path"))
        }
        Ok(())
    }

    pub fn has_valid_detailed_rules(&self) -> bool {
        !(self.regex.is_some() && (self.prefix.is_some() || self.suffix.is_some()))
    }
}

pub struct Config {
    pub watch: PathBuf,
    pub path: PathBuf,
    pub rules: Vec<Rule>,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        let app = Cli::new()?;
        let f = std::fs::File::open(&app.config)?;
        let reader = std::io::BufReader::new(f);
        let rules: Vec<Rule> = serde_json::from_reader(reader)?;
        let config = Config {
            watch: app.watch,
            path: app.config,
            rules,
        };
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), Error> {
        for rule in self.rules.iter() {
            rule.validate()?
        }
        Ok(())
    }

    pub fn map_extensions_to_rules(&self) -> HashMap<&String, &Rule> {
        let mut map: HashMap<&String, &Rule> = Default::default();
        for rule in self.rules.iter() {
            for extension in rule.extensions.as_ref().unwrap() {
               map.insert(extension, rule);
            }
        }
        map
    }
}
