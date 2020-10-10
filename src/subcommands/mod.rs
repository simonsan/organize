pub mod edit;
pub mod run;
pub mod watch;

#[derive(Clone, PartialEq, Debug)]
pub enum SubCommands {
    Edit,
    Run,
    Suggest,
    Watch,
    Logs,
}
