use crate::cli::Cli;
use std::fs;
use std::io::Error;
use std::ops::Index;
use yaml_rust::{Yaml, YamlLoader};
use crate::file::File;

pub struct Rule<'a> {
    yaml: &'a Yaml
}

impl <'a> Rule <'a>{
    pub fn from_yaml(yaml: &Yaml) -> Rule {
        Rule{ yaml }
    }
    pub fn get_dst_for_file(&self, file: &File) -> &'a str {
        let patterns = self.yaml["subpatterns"].as_vec();
        if patterns.is_some() {
            for pattern in patterns.unwrap() {
                if file.matches_pattern(pattern) {
                    let dst = pattern["dst"].as_str().unwrap();
                    return dst
                }
            }
        }

        self.yaml["dst"].as_str().unwrap()
    }

    pub fn is_badvalue(&self) -> bool {
        self.yaml.is_badvalue()
    }

    pub fn is_null(&self) -> bool {
        self.yaml.is_null()
    }
}

pub struct UserConfig {
    pub args: Cli,
    pub rules: Yaml,
}

impl UserConfig {
    pub fn new() -> Result<UserConfig, Error> {
        let cli = Cli::new()?;
        let content = fs::read_to_string(&cli.config)?;
        let rules = YamlLoader::load_from_str(content.as_str()).expect("error parsing config file");
        Ok(UserConfig {
            args: cli,
            rules: rules.index(0).clone(),
        })
    }
}
