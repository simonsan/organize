mod cli;
mod config;
mod file;
mod logger;
mod notifier;

#[macro_use]
extern crate clap;
use crate::config::Config;
use crate::notifier::Notifier;

fn main() {
    let config = Config::new();
    match config {
        Ok(config) => {
            let mut notifier = Notifier::new();
            notifier.watch(config);
        }
        Err(e) => eprintln!("{}", e),
    }
}
