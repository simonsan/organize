use crate::{
    kill_daemon,
    lock_file::LockFile,
    start_daemon,
    subcommands::{
        edit::{
            utils,
            UserConfig,
        },
        run::run,
        watch::Watcher,
        SubCommands,
    },
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

#[derive(Clone, Debug)]
pub struct Cli {
    pub subcommand: (SubCommands, ArgMatches),
}

impl Default for Cli {
    fn default() -> Self {
        let yaml = load_yaml!("../cli.yml");
        let app = App::from(yaml);
        let matches = app.get_matches();
        let (name, cmd) = matches.subcommand().unwrap();
        let cmd = cmd.clone(); // safe unwrap, a subcommand is mandatory
        let name = match name {
            "edit" => SubCommands::Edit,
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
}

impl Cli {
    pub fn run(self, config: UserConfig) -> Result<(), Error> {
        match self.subcommand.0 {
            SubCommands::Edit => {
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
                run(config.rules)?;
            }
            SubCommands::Suggest => todo!(),
            SubCommands::Watch => {
                if self.subcommand.1.is_present("replace") && self.subcommand.1.is_present("daemon") {
                    match kill_daemon() {
                        Ok(_) => {
                            let lock_file = LockFile::new();
                            lock_file.delete()?;
                            start_daemon()?;
                        }
                        Err(_) => {
                            println!("no running instance was found\nrun without --replace to start a new instance")
                        }
                    }
                } else if self.subcommand.1.is_present("daemon") && !self.subcommand.1.is_present("replace") {
                    let lock_file = LockFile::new();
                    if lock_file.path.exists() {
                        println!("a running instance already exists. Use `organizer stop` to stop this instance or `organizer watch --daemon --replace` to restart the daemon")
                    } else {
                        start_daemon()?;
                    }
                } else {
                    let mut watcher = Watcher::new();
                    watcher.watch(&config.rules, config.path);
                }
            }
            SubCommands::Logs => todo!(),
            SubCommands::Stop => {
                kill_daemon()?;
                let lock_file = LockFile::new();
                lock_file.delete()?;
            }
        };
        Ok(())
    }
}
