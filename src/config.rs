use crate::cli::Cli;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

fn vec_compare(a: Vec<&String>, b: Vec<&String>) -> bool {
    (a.len() == b.len()) &&  // zip stops at the shortest
        a.iter()
            .zip(b)
            .all(|(a,b)| a.as_str() == b.as_str())
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PatternRule {
    pub regex: String,
    pub dst: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Rule {
    pub extension: String,
    pub dst: String,
    pub patterns: Option<Vec<PatternRule>>,
}

impl Rule {
    fn validate(&self) -> Result<(), Error> {
        if self.extension == "" {
            return Err(Error::new(ErrorKind::InvalidData, "field 'extension' cannot be an empty string"))
        }
        if self.dst == "" {
            return Err(Error::new(ErrorKind::InvalidData, "field 'dst' cannot be an empty string"))
        }
        if self.patterns.is_some() {
            for pattern in self.patterns.as_ref().unwrap().iter() {
                if pattern.regex == "" {
                    return Err(Error::new(ErrorKind::InvalidData, "field 'regex' cannot be an empty string"))
                }
                if pattern.dst == "" {
                    return Err(Error::new(ErrorKind::InvalidData, "field 'dst' cannot be an empty string"))
                }
            }
        }
        Ok(())
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
        let mut config = Config {
            watch: app.watch,
            path: app.config,
            rules,
        };
        config.validate()?;
        Ok(config)
    }

    fn validate(&mut self) -> Result<(), Error> {
        let mut extensions: Vec<&String> = Vec::new();
        for rule in self.rules.iter() {
            rule.validate()?;
            extensions.push(&rule.extension);
        }

        let mut unique_extensions = extensions.clone();
        unique_extensions.dedup_by(|a, b| a == b);
        if !vec_compare(extensions, unique_extensions) {
            return Err(Error::new(ErrorKind::InvalidData, "ERROR: multiple rules with the same extension"))
        }
        Ok(())
    }

    pub fn map_extension_to_rule(&self) -> HashMap<&String, &Rule> {
        let mut map: HashMap<&String, &Rule> = HashMap::new();
        for rule in self.rules.iter() {
            map.insert(&rule.extension, rule);
        }
        map
    }
}


