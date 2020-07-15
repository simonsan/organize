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
                let config = config.validate()?;
                for rule in config.rules.iter() {
                    let actions = &rule.actions;
                    let folders = &rule.folders;
                    // let filters = &rule.filters; // these unwraps are safe because the config has been previously filled with defaults
                    let options = rule.options.as_ref().unwrap();
                    for folder in folders {
                        let files = fs::read_dir(folder.path.as_ref().unwrap()).unwrap();
                        for file in files {
                            match file {
                                Ok(file) => {
                                    let path = file.path();
                                    let filename = path.file_name().unwrap().to_str().unwrap();
                                    let is_file = path.is_file();
                                    let is_hidden_file = filename.starts_with(".");
                                    let must_process_hidden_files =
                                        folder.options.as_ref().unwrap().hidden_files.unwrap();
                                    if !(!is_file || !must_process_hidden_files && is_hidden_file) {
                                        // if it is a file and it's either a hidden file and this folder is configured to process them
                                        // or it's not a hidden file in the first place (automatically simplified)
                                        println!("{}", filename);
                                        println!("{:?}", file.metadata());
                                    }
                                }
                                Err(e) => eprintln!("{}", e),
                            }
                        }
                    }
                }
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
