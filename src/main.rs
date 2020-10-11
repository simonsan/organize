pub mod cli;
pub mod configuration;
pub mod file;
pub mod lock_file;
pub mod subcommands;

use crate::{
    cli::Cli,
    subcommands::edit::UserConfig,
};
use clap::crate_name;
use colored::Colorize;
use std::io::Error;

static PROJECT_NAME: &str = crate_name!();

fn main() -> Result<(), Error> {
    let mut cli = Cli::new();
    let config = UserConfig::new(&cli.subcommand.1);
    match config {
        Ok(config) => match cli.run(config) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => {
            eprint!("{}", "ERROR: ".red());
            eprintln!("{}", e);
        }
    }
    Ok(())
}
