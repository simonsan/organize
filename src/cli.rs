use crate::{
    config_path,
    subcommands::{
        config::edit,
        run::run,
        watch::{
            daemon::Daemon,
            watch,
        },
        SubCommands,
    },
    user_config::{
        rules::folder::Options,
        user_config::UserConfig,
    },
    PROJECT_NAME,
};
use clap::{
    load_yaml,
    App,
    ArgMatches,
};
use std::{
    env,
    io::Error,
};
use colored::Colorize;

#[derive(Clone, Debug)]
/// Struct that initializes the application and stores the main information about the subcommands and options introduced by the user
pub struct Cli {
    pub subcommand: (SubCommands, ArgMatches),
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
        let app = App::from(yaml);
        let matches = app.get_matches();
        let (name, cmd) = matches.subcommand().unwrap();
        let cmd = cmd.clone();
        let name = match name {
            "config" => SubCommands::Config,
            "run" => SubCommands::Run,
            "suggest" => SubCommands::Suggest,
            "watch" => SubCommands::Watch,
            "logs" => SubCommands::Logs,
            "stop" => SubCommands::Stop,
            _ => panic!("ERROR: unknown subcommand"),
        };

        Cli {
            subcommand: (name, cmd),
        }
    }

    pub fn run(self) -> Result<(), Error> {
        match self.subcommand.0 {
            SubCommands::Config => {
                if self.subcommand.1.is_present("show_path") {
                    println!("{}", config_path().display());
                } else if self.subcommand.1.is_present("show_defaults") {
                    let Options{ recursive, watch, ignore, suggestions, hidden_files } = Options::default();
                    println!("recursive: {}", recursive.to_string().purple());
                    println!("watch: {}", watch.to_string().purple());
                    println!("suggestions: {}", suggestions.to_string().purple());
                    println!("hidden_files: {}", hidden_files.to_string().purple());
                    println!("ignored_directories: {:?}", ignore);
                } else if self.subcommand.1.is_present("new") {
                    let config_file = env::current_dir()?.join(format!("{}.yml", PROJECT_NAME));
                    crate::utils::create_config_file(&config_file)?;
                    println!("New config file created at {}", config_file.display());
                } else {
                    let path = config_path();
                    edit(path)?;
                }
            }
            SubCommands::Run => {
                let config = UserConfig::new(&self.subcommand.1)?;
                run(config.rules, false)?
            }
            SubCommands::Suggest => todo!(),
            SubCommands::Watch => {
                let config = UserConfig::new(&self.subcommand.1)?;
                watch(self, &config)?
            }
            SubCommands::Logs => todo!(),
            SubCommands::Stop => {
                let daemon = Daemon::new();
                daemon.kill()?
            }
        };
        Ok(())
    }
}
