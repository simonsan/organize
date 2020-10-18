use crate::subcommands::{
    config::config,
    run::run,
    stop::stop,
    watch::watch,
    SubCommands,
};
use clap::{
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    load_yaml,
    App,
};
use std::{
    env,
    io::Error,
};

pub mod lock_file;
pub mod path;
pub mod string;
pub mod subcommands;
pub mod user_config;

static PROJECT_NAME: &str = crate_name!();

fn main() -> Result<(), Error> {
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

    match SubCommands::from(&args) {
        SubCommands::Config => config(&args),
        SubCommands::Run => run(&args),
        SubCommands::Watch => watch(&args),
        SubCommands::Stop => stop(),
        SubCommands::Suggest => unimplemented!(),
        SubCommands::Logs => unimplemented!(),
    }
}
