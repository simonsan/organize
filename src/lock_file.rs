use std::{
    env::temp_dir,
    fs,
    fs::OpenOptions,
    io::{
        prelude::*,
        Error,
    },
    path::{
        Path,
        PathBuf,
    },
};

use crate::{cli::Cli, commands::watch::daemon::Daemon, PROJECT_NAME};
use sysinfo::Pid;
use std::fs::File;

pub struct LockFile {
    path: PathBuf,
    sep: String,
}

impl LockFile {
    pub fn new() -> Self {
        LockFile {
            path: temp_dir().join(format!("{}.lock", PROJECT_NAME)),
            sep: "---".to_string(),
        }
    }

    fn section(&self, pid: &Pid, config: &Path) -> String {
        format!("{}\n{}\n{}", pid, config.display(), self.sep)
    }

    pub fn append(self, pid: Pid, config: &Path) -> Result<(), Error> {
        if !self.path.exists() {
            File::create(&self.path)?;
        }
        let mut f = OpenOptions::new().append(true).open(&self.path)?;
        writeln!(f, "{}", self.section(&pid, config))
    }

    pub fn get_running_watchers(&self) -> Vec<(Pid, PathBuf)> {
        let content = fs::read_to_string(&self.path);
        match content {
            Ok(content) => {
                if !content.is_empty() {
                    let content = content.trim().split(&self.sep);
                    let mut sections = Vec::new();
                    for section in content.into_iter().filter(|x| !x.is_empty() && x != &"\n") {
                        let section = section.lines().map(|x| x.to_string()).filter(|x| !x.is_empty()).collect::<Vec<_>>();
                        let pid = section.first().unwrap().parse().unwrap();
                        let path = section.get(1).unwrap().parse().unwrap();
                        sections.push((pid, path))
                    }
                    sections
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new()
        }
    }

    pub fn clear_dead_processes(&self, cli: &Cli) -> Result<(), Error> {
        let mut running_processes = String::new();
        for (pid, config) in self.get_running_watchers().iter() {
            let daemon = Daemon::new(cli, Some(*pid));
            if daemon.is_running() {
                running_processes.push_str(&self.section(pid, config));
                running_processes.push_str("\n");
            }
        }

        fs::write(&self.path, running_processes)?;
        Ok(())
    }

    pub fn find_process_by_path(&self, path: &Path) -> Option<(Pid, PathBuf)> {
        self.get_running_watchers()
            .iter()
            .filter(|(_, config)| config == &path)
            .collect::<Vec<_>>()
            .first()
            .map(|(pid, config)| (pid.clone(), config.clone()))
    }
}
