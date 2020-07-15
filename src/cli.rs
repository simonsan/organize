use crate::file::File;
use crate::subcommands::config::{utils, UserConfig};
use crate::subcommands::SubCommands;
use clap::{load_yaml, App, ArgMatches};
use std::io::Error;
use std::{env, fs};

#[derive(Clone, Debug)]
pub struct Cli {
    pub subcommand: (SubCommands, ArgMatches),
}

impl Cli {
    pub fn new() -> Self {
        let yaml = load_yaml!("../cli.yml");
        let app = App::from(yaml);
        let matches = app.get_matches();
        let (name, cmd) = matches.subcommand();
        let cmd = cmd.unwrap().clone(); // safe unwrap, a subcommand is mandatory
        let name = match name {
            "config" => SubCommands::Config,
            "run" => SubCommands::Run,
            "suggest" => SubCommands::Suggest,
            "watch" => SubCommands::Watch,
            "logs" => SubCommands::Logs,
            _ => panic!("ERROR: unknown subcommand"),
        };

        Cli {
            subcommand: (name, cmd),
        }
    }

    pub fn run(self, config: UserConfig) -> Result<(), Error> {
        match self.subcommand.0 {
            SubCommands::Config => {
                if self.subcommand.1.is_present("show_path") {
                    println!("{}", config.path.display());
                } else if self.subcommand.1.is_present("new") {
                    let config_file = env::current_dir()?.join("d-organizer.yml");
                    utils::create_config_file(&config_file)?;
                    println!("New config file created at {}", config_file.display());
                } else {
                    config.edit()?;
                }
            }
            SubCommands::Run => {
                // let config = config.validate()?;
                todo!();
            }
            SubCommands::Suggest => todo!(),
            SubCommands::Watch => {
                todo!();
                // if cli.subcommand.1.is_present("daemon") {
                //     start_daemon()
                // }
            }
            SubCommands::Logs => todo!(),
        };
        Ok(())
    }
}
