use crate::subcommands::{config::config, logs::logs, run::run, stop::stop, watch::watch};
use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, App, ArgMatches};
use lazy_static::lazy_static;
use std::{env, io::Result};

pub mod lock_file;
pub mod path;
pub mod string;
pub mod subcommands;
pub mod user_config;

lazy_static! {
    static ref MATCHES: ArgMatches = App::from(load_yaml!("cli.yml"))
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .name(crate_name!())
        .get_matches();
}

fn main() -> Result<()> {
    debug_assert!(MATCHES.subcommand().is_some());

    if cfg!(target_os = "windows") {
        eprintln!("Windows is not supported yet");
        return Ok(());
    }

    match MATCHES.subcommand_name().unwrap() {
        "config" => config(),
        "run" => run(),
        "watch" => watch(),
        "stop" => stop(),
        "logs" => logs(),
        _ => panic!("unknown subcommand"),
    }
}
