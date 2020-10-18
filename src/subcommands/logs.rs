use crate::user_config::UserConfig;
use chrono::prelude::Local;
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{Error, Write},
    path::PathBuf,
};

pub fn show_logs() -> Result<(), Error> {
    let logger = Logger::default();
    for line in logger.read_lines()? {
        // TODO: colorize line parts
        println!("{}", line)
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
        match level {
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
            Self::Debug => "debug",
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
        }
        .to_string()
    }
}

pub struct Logger {
    file: File,
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
        let file = OpenOptions::new().append(true).read(true).create_new(true).open(&path);

        match file {
            // open() may throw an AlreadyExists error when combined with create_new()
            Ok(file) => Self {
                file,
                path,
            },
            Err(_) => {
                let file = OpenOptions::new()
                    .append(true)
                    .read(true)
                    .open(&path)
                    .expect("could not open log file");
                Self {
                    file,
                    path,
                }
            }
        }
    }

    pub fn write(&mut self, level: Level, msg: &str) -> Result<(), Error> {
        let datetime = Local::now();
        let level = level.to_string().to_uppercase();
        writeln!(
            self.file,
            "[{}-{}] {}: {}",
            datetime.date(),
            datetime.time(),
            level,
            msg
        )
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
