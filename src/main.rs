mod cli;
mod config;
mod file;
mod logger;
pub mod utils;

#[macro_use]
extern crate clap;
extern crate yaml_rust;
use crate::cli::{Cli, SubCommands};
use crate::config::UserConfig;
use std::env;
use std::io::Error;
use std::process::Command;

fn main() -> Result<(), Error> {
    let config_file = dirs::home_dir()
        .expect("ERROR: cannot determine home directory")
        .join(".d-organizer")
        .join("old_config.yml");
    let yaml = load_yaml!("../cli.yml");
    let example_config = load_yaml!("../examples/old_config.yml");
    let cli = Cli::from_yaml(yaml);
    let config = UserConfig::new(&cli, &config_file);
    if !config_file.exists() {
        config
            .create_config_dir()
            .and_then(|config| config.create_config_file(example_config))?;
    }

    match cli.subcommand.0 {
        SubCommands::Config => {
            if cli.subcommand.1.is_present("show_path") {
                config.show_path();
            } else {
                config.edit_config()?;
            }
        }
        SubCommands::Run => {
            todo!();
            #[allow(unreachable_code)] // temporary
            if cli.daemon {
                start_daemon()
            }
        }
        SubCommands::Suggest => todo!(),
    }
    Ok(())
}

fn start_daemon() {
    let mut args = env::args();
    let command = args.next().unwrap(); // must've been started through a command
    let args: Vec<_> = args.filter(|arg| arg != "--daemon").collect();

    let pid = Command::new(command)
        .args(&args)
        .spawn()
        .expect("couldn't start daemon")
        .id();

    println!("daemon started with PID {}", pid);
}
