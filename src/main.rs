use std::io::Error;

use clap::crate_name;

use crate::cli::Cli;

pub mod cli;
pub mod commands;
pub mod file;
pub mod lock_file;
pub mod user_config;
pub mod utils;

static PROJECT_NAME: &str = crate_name!();

fn main() -> Result<(), Error> {
    let cli = Cli::new();
    match cli.run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
