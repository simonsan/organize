mod lib;

use std::{
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

use crate::{
    subcommands::watch::daemon::Daemon,
    user_config::UserConfig,
    PROJECT_NAME,
};
use std::fs::File;
use sysinfo::Pid;

pub struct LockFile {
    path: PathBuf,
    sep: String,
}

impl LockFile {
    pub fn new() -> Result<Self, Error> {
        let path = UserConfig::dir().join(format!("{}.lock", PROJECT_NAME));
        if !path.exists() {
            File::create(&path).expect("could not create lock file");
        }
        let f = LockFile {
            path,
            sep: "---".into(),
        };
        f.clear_dead_processes()
    }

    fn section(&self, pid: &Pid, config: &Path) -> String {
        format!("{}\n{}\n{}", pid, config.display(), self.sep)
    }

    fn set_readonly(&self, readonly: bool) -> Result<(), Error> {
        let f = File::open(&self.path)?;
        let mut perms = f.metadata()?.permissions();
        perms.set_readonly(readonly);
        f.set_permissions(perms)?;
        Ok(())
    }

    pub fn append(&self, pid: Pid, config: &Path) -> Result<(), Error> {
        if !self.path.exists() {
            File::create(&self.path)?;
        }
        self.set_readonly(false)?;
        let mut f = OpenOptions::new().append(true).open(&self.path)?;
        let result = writeln!(f, "{}", self.section(&pid, config));
        self.set_readonly(true)?;
        result
    }

    pub fn get_running_watchers(&self) -> Vec<(Pid, PathBuf)> {
        let content = fs::read_to_string(&self.path);
        match content {
            Ok(content) => {
                if !content.is_empty() {
                    let content = content.trim().split(&self.sep);
                    content
                        .filter(|section| !section.is_empty() && *section != "\n")
                        .map(|section| {
                            let section = section
                                .lines()
                                .map(|line| line.to_string())
                                .filter(|line| !line.is_empty())
                                .collect::<Vec<_>>();
                            let pid = section.first().unwrap().parse().unwrap();
                            let path = section.get(1).unwrap().parse().unwrap();
                            (pid, path)
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
        }
    }

    fn clear_dead_processes(self) -> Result<Self, Error> {
        self.set_readonly(false)?;
        let mut running_processes = String::new();
        for (pid, config) in self.get_running_watchers().iter() {
            let daemon = Daemon::new(Some(*pid));
            if daemon.is_running() {
                running_processes.push_str(&self.section(pid, config));
                running_processes.push_str("\n");
            }
        }
        fs::write(&self.path, running_processes)?;
        self.set_readonly(true)?;
        Ok(self)
    }

    pub fn find_process_by_path(&self, path: &Path) -> Option<(Pid, PathBuf)> {
        self.get_running_watchers()
            .iter()
            .filter(|(_, config)| config == path)
            .collect::<Vec<_>>()
            .first()
            .map(|(pid, config)| (*pid, config.clone()))
    }

    pub fn find_process_by_pid(&self, pid: Pid) -> Option<(Pid, PathBuf)> {
        self.get_running_watchers()
            .iter()
            .filter(|(running_pid, _)| pid == *running_pid)
            .collect::<Vec<_>>()
            .first()
            .map(|(pid, config)| (*pid, config.clone()))
    }
}
