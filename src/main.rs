pub mod cli;
pub mod configuration;
pub mod file;
pub mod lock_file;
pub mod subcommands;

use crate::cli::Cli;
use clap::crate_name;
use dirs::home_dir;
use std::{
    io::Error,
    path::PathBuf,
};

static PROJECT_NAME: &str = crate_name!();

pub fn project_dir() -> PathBuf {
    home_dir()
        .expect("ERROR: cannot determine home directory")
        .join(format!(".{}", PROJECT_NAME))
}

fn main() -> Result<(), Error> {
    let cli = Cli::new();
    match cli.run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
