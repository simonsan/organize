pub(crate) mod config;
mod logs;

#[derive(Clone, PartialEq, Debug)]
pub enum SubCommands {
    Config,
    Run,
    Suggest,
    Watch,
    Logs,
}
