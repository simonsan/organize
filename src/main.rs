pub mod cli;
pub mod configuration;
pub mod file;
pub mod subcommands;

use crate::{
    cli::Cli,
    subcommands::config::UserConfig,
};
use colored::Colorize;
use std::{
    env,
    process::Command,
};

// TODO remove panics

fn main() {
    let cli = Cli::default();
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

#[allow(dead_code)]
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
