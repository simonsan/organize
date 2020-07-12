mod cli;
mod config;
mod file;
mod logger;
mod notifier;

#[macro_use]
extern crate clap;
use crate::config::Config;
use crate::notifier::Notifier;
use std::env;
use std::process::Command;

fn main() {
    let config = Config::new();
    match config {
        Ok(config) if config.args.daemon => start_daemon(),
        Ok(config) => {
            let mut notifier = Notifier::new();
            notifier.watch(config);
        }
        Err(e) => eprintln!("{}", e),
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
