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
    let text = logger.read()?;
    let re = r"(?P<time>\[.+]) (?P<level>[A-Z]+?): (?:\()(?P<action>\w+?)(?:\)) (?P<old_path>.+?) (?:(?P<sep>->) (?P<new_path>.+))?";
    let re = Regex::new(re).unwrap();
    for r#match in re.captures_iter(&text) {
        print!(
            "{} {}: ({}) {}",
            &r#match["time"].dimmed(),
            Level::from(&r#match["level"]).colored(),
            &r#match["action"].bold(),
            &r#match["old_path"].underline(),
        );
        if let (Some(sep), Some(new_path)) = (r#match.name("sep"), r#match.name("new_path")) {
            println!(" {} {}", sep.as_str(), new_path.as_str().underline())
        } else {
            println!()
        }
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

    pub fn read(&self) -> Result<String, Error> {
        fs::read_to_string(&self.path)
    }
}
