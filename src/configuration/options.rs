use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Options {
    pub recursive: bool,
    pub watch: bool,
    pub ignore: Vec<PathBuf>,
    pub suggestions: bool,
    pub enabled: bool,
    pub system_files: bool,
    pub hidden_files: bool,
}
