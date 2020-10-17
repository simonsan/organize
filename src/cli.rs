use std::io::Error;

use clap::{
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    load_yaml,
    App,
    ArgMatches,
};

use crate::{
    commands::{
        config::config,
        run::run,
        stop::stop,
        watch::watch,
        SubCommands,
    },
    lock_file::LockFile,
    user_config::UserConfig,
};

#[derive(Clone, Debug)]
/// Struct that initializes the application and stores the main information about the subcommands and options introduced by the user
pub struct Cli {
    pub subcommand: SubCommands,
    pub args: ArgMatches,
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}

impl Cli {
    /// Creates a new CLI instance that initializes the app
    pub fn new() -> Self {
        let yaml = load_yaml!("../cli.yml");
        let app = App::from(yaml)
            .author(crate_authors!())
            .about(crate_description!())
            .version(crate_version!())
            .name(crate_name!());

        let matches = app.get_matches();
        let (subcommand, args) = matches.subcommand().unwrap();
        let args = args.clone();
        let subcommand = match subcommand {
            "config" => SubCommands::Config,
            "run" => SubCommands::Run,
            "suggest" => SubCommands::Suggest,
            "watch" => SubCommands::Watch,
            "logs" => SubCommands::Logs,
            "stop" => SubCommands::Stop,
            _ => panic!("ERROR: unknown subcommand"),
        };

        Cli {
            subcommand,
            args,
        }
    }

    pub fn run(self) -> Result<(), Error> {
        match self.subcommand {
            SubCommands::Config => {
                config(self)?;
            }
            SubCommands::Run => {
                let lock_file = LockFile::new();
                lock_file.clear_dead_processes()?;
                let config = UserConfig::new(&self)?;
                run(&config, false)?;
            }
            SubCommands::Watch => {
                let lock_file = LockFile::new();
                lock_file.clear_dead_processes()?;
                if self.subcommand == SubCommands::Watch {
                    watch(self)?
                }
            }
            SubCommands::Suggest => todo!(),
            SubCommands::Logs => todo!(),
            SubCommands::Stop => {
                stop()?;
            }
        }
        Ok(())
    }
}
