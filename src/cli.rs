use std::{
    env,
    io::Error,
    process,
};

use clap::{
    load_yaml,
    App,
    ArgMatches,
};
use colored::Colorize;
use dialoguer::{
    Confirm,
    MultiSelect,
};

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
use dialoguer::theme::ColorfulTheme;

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
                let lock_file = LockFile::new();
                lock_file.clear_dead_processes()?;
                let _config = UserConfig::path(&self);
                let watchers = lock_file.get_running_watchers();
                let pids = watchers.iter().map(|(pid, _)| pid).collect::<Vec<_>>();
                let paths = watchers.iter().map(|(_, path)| path.display()).collect::<Vec<_>>();

                if watchers.is_empty() {
                    println!("No instance was found running.");
                    let prompt = "Would you like to start a new daemon with the default configuration?";
                    let confirm = Confirm::new().with_prompt(prompt).interact();
                    if confirm.is_ok() && confirm.unwrap() {
                        let daemon = Daemon::new(process::id() as i32);
                        daemon.start();
                    }
                } else if watchers.len() == 1 {
                    let daemon = Daemon::new(**pids.first().unwrap());
                    daemon.kill();
                } else {
                    let prompt = "Press Spacebar to select one or more options and press Enter to stop them:";
                    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                        .with_prompt(prompt)
                        .items(&paths[..])
                        .interact()
                        .unwrap();
                    for selection in selections {
                        let daemon = Daemon::new(**pids.get(selection).unwrap());
                        daemon.kill();
                    }
                }
            }
        }
        Ok(())
    }
}
