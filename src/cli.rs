use std::{
    env,
    io::{
        Error,
        ErrorKind,
    },
};

use clap::{
    load_yaml,
    App,
    ArgMatches,
};
use colored::Colorize;

use crate::{
    commands::{
        config::edit,
        run::run,
        watch::{
            daemon::Daemon,
            watch,
        },
        SubCommands,
    },
    lock_file::LockFile,
    user_config::{
        rules::folder::Options,
        UserConfig,
    },
    PROJECT_NAME,
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
                    println!("{}", UserConfig::path(&self).display());
                } else if self.args.is_present("show_defaults") {
                    let Options {
                        recursive,
                        watch,
                        ignore,
                        suggestions,
                        hidden_files,
                    } = Options::default();
                    println!("recursive: {}", recursive.to_string().purple());
                    println!("watch: {}", watch.to_string().purple());
                    println!("suggestions: {}", suggestions.to_string().purple());
                    println!("hidden_files: {}", hidden_files.to_string().purple());
                    println!("ignored_directories: {:?}", ignore);
                } else if self.args.is_present("new") {
                    let config_file = env::current_dir()?.join(format!("{}.yml", PROJECT_NAME));
                    UserConfig::create(&config_file)?;
                    println!("New config file created at {}", config_file.display());
                } else {
                    edit(UserConfig::path(&self))?;
                }
            }
            SubCommands::Run => {
                let lock_file = LockFile::new();
                lock_file.clear_dead_processes()?;
                let config = UserConfig::new(&self)?;
                run(&config.rules, false)?;
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
                let lock_file = LockFile::new();
                let path = UserConfig::path(&self);
                if self.args.is_present("with_config") {
                    match lock_file.find_process_by_path(&path) {
                        Some((pid, _)) => {
                            let daemon = Daemon::new(pid);
                            if daemon.is_running() {
                                daemon.kill();
                            }
                        }
                        None => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "no instance was found with the desired configuration\n\
                            Run '{} stop' to stop all instances or rerun the last command with a different path",
                            ))
                        }
                    }
                } else {
                    for (pid, _) in lock_file.get_running_watchers() {
                        let daemon = Daemon::new(pid);
                        if daemon.is_running() {
                            daemon.kill()
                        }
                    }
                }
                lock_file.clear_dead_processes()?;
            }
        };
        Ok(())
    }
}
