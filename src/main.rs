use crate::cli::Cli;
use clap::crate_name;
use std::{
    env,
    io::Error,
};

pub mod cli;
pub mod commands;
pub mod file;
pub mod lock_file;
pub mod path;
pub mod user_config;
pub mod utils;

static PROJECT_NAME: &str = crate_name!();

fn main() -> Result<(), Error> {
    if env::consts::OS == "windows" {
        eprintln!("Windows is not supported yet");
        return Ok(());
    }
    let cli = Cli::new();
    match cli.run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
