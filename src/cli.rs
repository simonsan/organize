use clap::{App, ArgMatches};
use yaml_rust::Yaml;

#[derive(PartialEq, Debug)]
pub enum SubCommands {
    Config,
    Run,
    Suggest,
    Watch,
}

pub struct Cli<'a> {
    pub subcommand: (SubCommands, ArgMatches<'a>),
    pub daemon:     bool,
}
impl<'a> Cli<'a> {
    pub fn from_yaml(yaml: &'a Yaml) -> Self {
        let app = App::from_yaml(yaml);
        let matches = app.get_matches_safe().unwrap_or_else(|e| e.exit());
        let (name, cmd) = matches.subcommand();
        let cmd = cmd.unwrap().clone();
        let name = match name {
            "config" => SubCommands::Config,
            "run" => SubCommands::Run,
            "suggest" => SubCommands::Suggest,
            "watch" => SubCommands::Watch,
            _ => panic!("ERROR: subcommands are no longer required or match arms are missing"),
        };

        Cli {
            subcommand: (name, cmd),
            daemon:     false, // temporary
        }
    }
}
