use crate::cli::Cli;
use serde::Deserialize;
use std::io::{BufReader, Error};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub extensions: Option<Vec<String>>,
    pub suffix: Option<String>,
    pub prefix: Option<String>,
    pub regex: Option<String>,
    pub dst: String,
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
        let rules: Result<Vec<Rule>, serde_json::Error> = serde_json::from_reader(reader);
        match rules {
            Ok(rules) => Ok(Config {
                watch: app.watch,
                path: app.config,
                rules,
            }),
            Err(e) => Err(Error::from(e)),
        }
    }
}
