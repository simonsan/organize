pub mod cli;
pub mod configuration;
pub mod daemon;
pub mod file;
pub mod lock_file;
pub mod subcommands;

use crate::{
    cli::Cli,
    subcommands::edit::UserConfig,
};
use colored::Colorize;
use std::io::Error;

// TODO remove panics
fn main() -> Result<(), Error> {
    let mut cli = Cli::default();
    let config = UserConfig::new(&cli);
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
