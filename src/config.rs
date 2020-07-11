use crate::cli::Cli;
use crate::file::File;
use std::fs;
use std::io::{Error, ErrorKind};
use serde::{Deserialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Pattern {
    pub(crate) regex: String,
    pub(crate) new_folder: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Fields {
    pub(crate) new_folder: String,
    pub(crate) patterns: Option<Vec<Pattern>>,
}

pub struct Rule<'a> {
    fields: &'a Fields
}

impl <'a> Rule<'a> {
    pub fn from_fields(fields: &'a Fields) -> Self {
       Rule{fields}
    }
    pub fn get_dst_for_file(&self, file: &File) -> String {
        let patterns = &self.fields.patterns;
        if patterns.is_some() {
            for pattern in patterns.as_ref().unwrap() {
                if file.matches_pattern(&pattern) {
                    return pattern.new_folder.to_owned()
                }
            }
        }
        self.fields.new_folder.to_owned()
    }

}

pub type Rules = HashMap<String, Fields>;
#[derive(Debug, PartialEq, Deserialize)]
struct UserConfig {
    rules: Rules,
}

pub struct Config {
    pub rules: Rules,
    pub args: Cli,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let cli = Cli::new()?;
        let content = fs::read_to_string(&cli.config)?;
        let user_config: UserConfig = serde_yaml::from_str(content.as_str()).expect("error parsing config file");
        let config = Config {
            args: cli,
            rules: user_config.rules,
        };

        Ok(config.validate()?)
    }

    fn validate(self) -> Result<Self, Error> {
        for (_, fields) in self.rules.iter() {
            if fields.new_folder == "" {
                return Err(Error::new(ErrorKind::InvalidData, "field 'new_folder' cannot be an empty string"));
            }
            let new_path = Path::new(&fields.new_folder);
            if !new_path.exists() {
                return Err(Error::new(ErrorKind::InvalidData, format!("specified path '{}' does not exist", fields.new_folder)));
            }
            match &fields.patterns {
                Some(patterns) => {
                   for pattern in patterns.iter() {
                       if pattern.regex == "" {
                           return Err(Error::new(ErrorKind::InvalidData, "field 'regex' cannot be an empty string"));
                       }
                       if pattern.new_folder == "" {
                           return Err(Error::new(ErrorKind::InvalidData, "field 'new_folder' cannot be an empty string"));
                       }
                       let new_path = Path::new(&pattern.new_folder);
                       if !new_path.exists() {
                           return Err(Error::new(ErrorKind::InvalidData, format!("specified path '{}' does not exist", pattern.new_folder)));
                       }
                   }
                },
                None => {
                    continue
                }
            }
        }
        Ok(self)
    }
}
