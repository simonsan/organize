mod cli;
pub mod configuration;
pub mod file;
mod subcommands;

use crate::cli::Cli;
use crate::subcommands::config::UserConfig;
use colored::Colorize;
use std::env;
use std::io::Error;
use std::process::Command;

fn main() {
    let cli = Cli::new();
    let config = UserConfig::new(&cli);
    match config {
        Ok(config) => cli.run(config).unwrap(),
        Err(e) => {
            let description = e.to_string();
            print!("{}", "ERROR: ".red());
            println!("{}", description);
        }
    }
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
