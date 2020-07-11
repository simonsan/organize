mod cli;
mod config;
mod file;
mod logger;
mod notifier;

extern crate log;
#[macro_use]
extern crate clap;
use crate::config::UserConfig;
use crate::notifier::Notifier;

fn main() {
    let user_config = UserConfig::new();
    match user_config {
        Ok(config) => {
            let mut notifier = Notifier::new();
            notifier.watch(config);
        }
        Err(e) => eprintln!("{}", e),
    }
}
