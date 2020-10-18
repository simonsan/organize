use crate::user_config::UserConfig;
use chrono::prelude::Local;
use colored::{ColoredString, Colorize};
use regex::Regex;
use std::{
    fs,
    fs::OpenOptions,
    io::{Error, Write},
    path::PathBuf,
};

pub fn show_logs() -> Result<(), Error> {
    let logger = Logger::default();
    for line in logger.read_lines()? {
        // let time = Regex::new("\\[.+]").unwrap().find(&line).unwrap().as_str().to_string();
        let mut components = line.split(" ");
        let time = components.next().unwrap().to_string();
        let level = components.next().unwrap().to_string().replace(":", "");
        let level = Level::from(level.as_str()).colored();
        let mut action = components.next().unwrap().to_string();
        action = action.trim_start_matches('(').trim_end_matches(')').to_string();
        let as_str = components.collect::<String>();
        components = as_str.split("->");
        let first_path = components.next().unwrap().to_string();
        let last_path = components.next().unwrap().to_string();

        println!(
            "{} {}: ({}) {} -> {}",
            time.dimmed(),
            level,
            action.bold(),
            first_path.underline(),
            last_path.underline()
        )
    }
    Ok(())
}

pub enum Level {
    Debug,
    Warn,
    Info,
    Error,
}

impl From<&str> for Level {
    fn from(level: &str) -> Self {
        let level = level.to_lowercase();
        match level.as_str() {
            "debug" => Self::Debug,
            "error" => Self::Error,
            "warn" => Self::Warn,
            "info" => Self::Info,
            _ => panic!("unknown log level"),
        }
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Self::Debug => "DEBUG",
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
        }
        .to_string()
    }
}

impl Level {
    pub fn colored(&self) -> ColoredString {
        match self {
            Level::Info => self.to_string().green(),
            Level::Error => self.to_string().red(),
            Level::Warn => self.to_string().yellow(),
            Level::Debug => self.to_string().cyan(),
        }
    }
}

pub struct Logger {
    path: PathBuf,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(Self::default_path())
    }
}

impl Logger {
    pub fn default_path() -> PathBuf {
        UserConfig::dir().join(".log")
    }

    pub fn new(path: PathBuf) -> Self {
        OpenOptions::new().append(true).create_new(true).open(&path).ok();
        Self {
            path,
        }
    }

    pub fn write(&mut self, level: Level, msg: &str) -> Result<(), Error> {
        let datetime = Local::now();
        let level = level.to_string().to_uppercase();
        let file = OpenOptions::new().append(true).open(&self.path)?;
        writeln!(&file, "[{}-{}] {}: {}", datetime.date(), datetime.time(), level, msg)
    }

    pub fn len(&self) -> usize {
        self.read_lines().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn delete(self) -> Result<(), Error> {
        fs::remove_file(self.path)
    }

    pub fn read_lines(&self) -> Result<Vec<String>, Error> {
        let logs = fs::read_to_string(&self.path)?;
        Ok(logs.lines().map(|str| str.to_string()).collect::<Vec<_>>())
    }
}
