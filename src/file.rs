use std::path::{PathBuf, Path};
use crate::config::Rule;
use regex::Regex;
use std::io::Error;

pub struct File<'a> {
    path: &'a PathBuf
}

impl <'a> File<'a> {
    pub fn from(path: &PathBuf) -> File {
       File{ path }
    }

    fn matches_prefix_or_suffix(&self, prefix: &Option<String>, suffix: &Option<String>) -> bool {
        let mut regex = String::new();
        if prefix.is_some() {
            // let regex = Regex::new(format!(r"{}.*{}", prefix.as_ref().unwrap(), suffix.as_ref().unwrap()).as_str())
            //     .expect("invalid prefix or suffix");
            // return regex.is_match(self.path.to_str().unwrap());
            regex += prefix.as_ref().unwrap().as_str()
        }
        regex += ".*";
        if suffix.is_some() {
            regex += suffix.as_ref().unwrap().as_str()
        }
        let regex = Regex::new(regex.as_str())
            .expect("invalid regex");
        regex.is_match(self.path.to_str().unwrap())
    }

    pub fn matches_detailed_rules(&self, rule: &Rule) -> bool {
        if self.matches_prefix_or_suffix(&rule.prefix, &rule.suffix) {
            return true;
        } else if rule.regex.is_some() {
            let regex = Regex::new(rule.regex.as_ref().unwrap())
                .expect("invalid regex");
            return regex.is_match(self.path.to_str().unwrap());
        }
        false
    }

    pub fn rename(&self, rule: &Rule) -> Result<(), Error> {
        Ok(std::fs::rename(self.path.to_str().unwrap(), Path::new(&rule.dst).join(self.path.file_name().unwrap()).to_str().unwrap())?)
    }
}