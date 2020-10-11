use crate::{
    daemon::Daemon,
    lock_file::LockFile,
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
    io::{
        Error,
        ErrorKind,
    },
};
use sysinfo::{
    System,
    SystemExt,
};

#[derive(Clone, Debug)]
pub struct Cli {
    pub subcommand: (SubCommands, ArgMatches),
    pub running_daemon: bool, /* running `organizer` with the --daemon option involves running the `run` function twice, so we need this attribute to keep track of which iteration we're in */
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
            running_daemon: false,
        }
    }
}

impl Cli {
    pub fn is_running(&self) -> bool {
        let lock_file = LockFile::new();
        let pid = lock_file.get_pid();
        if pid.is_err() {
            panic!("no lock file found");
        }
        let sys = System::new_all();
        sys.get_processes().get(&pid.unwrap()).is_some() && !self.running_daemon
    }

    pub fn run(&mut self, config: UserConfig) -> Result<(), Error> {
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
                    Daemon::new().restart()?;
                }

                let lock_file = LockFile::new();
                if self.is_running() {
                    return Err(
                        Error::new(
                            ErrorKind::Other,
                            "a running instance already exists. Use `organizer stop` to stop this instance or `organizer watch --daemon --replace` to restart the daemon"
                        )
                    );
                }
                if self.subcommand.1.is_present("daemon") {
                    let pid = Daemon::new().start()?;
                    lock_file.write_pid(pid)?;
                    self.running_daemon = true;
                } else {
                    let mut watcher = Watcher::new();
                    watcher.watch(&config.rules, config.path);
                }
            }
            SubCommands::Logs => todo!(),
            SubCommands::Stop => {
                let daemon = Daemon::new();
                daemon.kill()?;
            }
        };
        Ok(())
    }
}
