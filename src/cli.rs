use crate::{
    subcommands::{
        edit::{
            utils,
            UserConfig,
        },
        run::run,
        watch::{
            daemon::Daemon,
            watch,
        },
        SubCommands,
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

#[derive(Clone, Debug)]
pub struct Cli {
    pub subcommand: (SubCommands, ArgMatches),
    pub daemon: Daemon,
    pub is_running: bool,
    /* running `organizer` with the --daemon option involves running the `run` function twice,
     * so we need this attribute to keep track of which iteration we're in */
}

impl Default for Cli {
    fn default() -> Self {
        Cli::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        let yaml = load_yaml!("../cli.yml");
        let app = App::from(yaml);
        let matches = app.get_matches();
        let (name, cmd) = matches.subcommand().unwrap();
        let cmd = cmd.clone();
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
            daemon: Daemon::new(),
            is_running: false,
        }
    }

    pub fn run(&mut self, config: UserConfig) -> Result<(), Error> {
        match self.subcommand.0 {
            SubCommands::Edit => {
                if self.subcommand.1.is_present("show_path") {
                    println!("{}", config.path.display());
                } else if self.subcommand.1.is_present("new") {
                    let config_file = env::current_dir()?.join(format!("{}.yml", PROJECT_NAME));
                    utils::create_config_file(&config_file)?;
                    println!("New config file created at {}", config_file.display());
                } else {
                    config.edit()?;
                }
            }
            SubCommands::Run => {
                run(config.rules)?;
            }
            SubCommands::Suggest => todo!(),
            SubCommands::Watch => watch(self, &config)?,
            SubCommands::Logs => todo!(),
            SubCommands::Stop => self.daemon.kill()?,
        };
        Ok(())
    }
}
