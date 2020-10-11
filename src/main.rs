pub mod cli;
pub mod configuration;
pub mod file;
pub mod subcommands;
pub mod lock_file;

use crate::{
    cli::Cli,
    subcommands::edit::UserConfig,
};
use colored::Colorize;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut cli = Cli::new();
    let config = UserConfig::new(&cli.subcommand.1);
    match config {
        Ok(config) => match cli.run(config) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e.to_string()),
        },
        Err(e) => {
            let description = e.to_string();
            print!("{}", "ERROR: ".red());
            println!("{}", description);
        }
    }
    Ok(())
}
