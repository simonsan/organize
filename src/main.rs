use crate::subcommands::{config::config, logs::logs, run::run, stop::stop, watch::watch, SubCommands};
use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, App};
use std::{
    env,
    io::Result,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod lock_file;
pub mod path;
pub mod string;
pub mod subcommands;
pub mod user_config;

static WATCHING: AtomicBool = AtomicBool::new(false);

fn main() -> Result<()> {
    if env::consts::OS == "windows" {
        eprintln!("Windows is not supported yet");
        return Ok(());
    }

    let args = App::from(load_yaml!("cli.yml"))
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .name(crate_name!())
        .get_matches();

    WATCHING.store(
        SubCommands::from(args.subcommand_name().unwrap()) == SubCommands::Watch,
        Ordering::SeqCst,
    );

    match SubCommands::from(&args) {
        SubCommands::Config => config(&args),
        SubCommands::Run => run(&args),
        SubCommands::Watch => watch(&args),
        SubCommands::Stop => stop(),
        SubCommands::Logs => logs(&args),
    }
}
