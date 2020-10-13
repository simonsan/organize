pub mod cli;
pub mod file;
pub mod lock_file;
pub mod commands;
pub mod user_config;
pub mod utils;

use crate::cli::Cli;
use clap::crate_name;
use dirs::home_dir;
use std::{
    io::Error,
    path::PathBuf,
};

static PROJECT_NAME: &str = crate_name!();

pub fn config_directory() -> PathBuf {
    home_dir()
        .expect("ERROR: cannot determine home directory")
        .join(format!(".{}", PROJECT_NAME))
}

pub fn config_path() -> PathBuf {
    config_directory().join("config.yml")
}

fn main() -> Result<(), Error> {
    let cli = Cli::new();
    match cli.run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
