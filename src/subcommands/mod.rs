use clap::ArgMatches;

pub mod config;
pub mod run;
pub mod stop;
pub mod watch;

#[derive(Clone, PartialEq, Debug)]
pub enum SubCommands {
    Config,
    Run,
    Suggest,
    Watch,
    Logs,
    Stop,
}

impl ToString for SubCommands {
    fn to_string(&self) -> String {
        match self {
            SubCommands::Config => "config".into(),
            SubCommands::Run => "run".into(),
            SubCommands::Suggest => "suggest".into(),
            SubCommands::Watch => "watch".into(),
            SubCommands::Logs => "logs".into(),
            SubCommands::Stop => "stop".into(),
        }
    }
}

impl From<&ArgMatches> for SubCommands {
    fn from(args: &ArgMatches) -> Self {
        Self::from(args.subcommand_name().unwrap())
    }
}
impl From<&str> for SubCommands {
    fn from(name: &str) -> Self {
        match name {
            "config" => SubCommands::Config,
            "run" => SubCommands::Run,
            "suggest" => SubCommands::Suggest,
            "watch" => SubCommands::Watch,
            "logs" => SubCommands::Logs,
            "stop" => SubCommands::Stop,
            _ => panic!("unknown subcommand"),
        }
    }
}
