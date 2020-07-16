pub(crate) mod config;
mod logs;
pub mod run;

#[derive(Clone, PartialEq, Debug)]
pub enum SubCommands {
    Config,
    Run,
    Suggest,
    Watch,
    Logs,
}
