use std::{
    env,
    io::Error,
};
use std::path::PathBuf;

use clap::{
    App,
    ArgMatches,
    load_yaml,
};
use colored::Colorize;
use dirs::home_dir;

use crate::{
    commands::{
        config::edit,
        run::run,
        SubCommands,
        watch::{
            daemon::Daemon,
            watch,
        },
    },
    PROJECT_NAME,
    user_config::{
        rules::folder::Options,
        user_config::UserConfig,
    },
};

pub fn config_directory() -> PathBuf {
    home_dir()
        .expect("ERROR: cannot determine home directory")
        .join(format!(".{}", PROJECT_NAME))
}

pub fn config_path(cli: &Cli) -> PathBuf {
    match cli.args.value_of("with_config") {
        Some(path) => PathBuf::from(path),
        None => config_directory().join("config.yml"),
    }
}

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
        let app = App::from(yaml);
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
                if self.args.is_present("show_path") {
                    println!("{}", config_path(&self).display());
                } else if self.args.is_present("show_defaults") {
                    let Options { recursive, watch, ignore, suggestions, hidden_files } = Options::default();
                    println!("recursive: {}", recursive.to_string().purple());
                    println!("watch: {}", watch.to_string().purple());
                    println!("suggestions: {}", suggestions.to_string().purple());
                    println!("hidden_files: {}", hidden_files.to_string().purple());
                    println!("ignored_directories: {:?}", ignore);
                } else if self.args.is_present("new") {
                    let config_file = env::current_dir()?.join(format!("{}.yml", PROJECT_NAME));
                    crate::utils::create_config_file(&config_file)?;
                    println!("New config file created at {}", config_file.display());
                } else {
                    let path = config_path(&self);
                    edit(path)?;
                }
            }
            SubCommands::Run => {
                let config = UserConfig::new(&self)?;
                run(config.rules, false)?
            }
            SubCommands::Suggest => todo!(),
            SubCommands::Watch => {
                let config = UserConfig::new(&self)?;
                watch(self, config)?
            }
            SubCommands::Logs => todo!(),
            SubCommands::Stop => {
                let daemon = Daemon::new(&self);
                daemon.kill()?
            }
        };
        Ok(())
    }
}
