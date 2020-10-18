use clap::ArgMatches;

pub mod config;
pub mod logs;
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

impl From<String> for SubCommands {
    fn from(name: String) -> Self {
        let name = name.as_str();
        Self::from(name)
    }
}

impl From<&str> for SubCommands {
    fn from(name: &str) -> Self {
        match name {
            "config" => Self::Config,
            "run" => Self::Run,
            "suggest" => Self::Suggest,
            "watch" => Self::Watch,
            "logs" => Self::Logs,
            "stop" => Self::Stop,
            _ => panic!("unknown subcommand"),
        }
    }
}
