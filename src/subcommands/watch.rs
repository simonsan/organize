use std::{
    io::Result,
    process,
    sync::mpsc::{channel, Receiver},
};

use colored::Colorize;
use notify::{op, raw_watcher, RawEvent, RecommendedWatcher, RecursiveMode, Watcher as OtherWatcher};

use crate::{
    lock_file::LockFile,
    path::MatchesFilters,
    subcommands::{run::run, stop::stop},
    user_config::{rules::folder::Options, UserConfig},
    MATCHES,
};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use sysinfo::{ProcessExt, RefreshKind, Signal, System, SystemExt};

pub fn watch() -> Result<()> {
    let lock_file = LockFile::new()?;

    // REPLACE
    if MATCHES.subcommand().unwrap().1.is_present("replace") {
        Daemon::replace()?;
    } else if lock_file.get_running_watchers().is_empty() {
        let path = UserConfig::path();
        run()?;
        lock_file.append(process::id() as i32, &path)?;
        std::mem::drop(path);
        std::mem::drop(lock_file);
        let mut watcher = Watcher::new();
        watcher.run()?;
    } else {
        let options = ["Stop instance", "Run anyway", "Do nothing"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("An existing instance was found:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => stop()?,
            1 => {
                run()?;
                lock_file.append(process::id() as i32, &path)?;
                let mut watcher = Watcher::new();
                watcher.run()?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub struct Watcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<RawEvent>,
}

impl Default for Watcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Watcher {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let watcher = raw_watcher(sender).unwrap();
        Watcher {
            watcher,
            receiver,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let config = UserConfig::new()?;
        for rule in config.rules.iter() {
            for folder in rule.folders.iter() {
                let is_recursive = if folder.options.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                };
                self.watcher.watch(&folder.path, is_recursive).unwrap();
            }
        }

        // PROCESS SIGNALS
        let path2rules = config.to_map();
        loop {
            if let Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                ..
            }) = self.receiver.recv()
            {
                if let op::CREATE = op {
                    if path.is_file() {
                        let parent = path.parent().unwrap().to_path_buf();
                        let values = path2rules.get(&parent).unwrap().to_owned();
                        'rules: for (rule, i) in values {
                            let folder = rule.folders.get(i).unwrap();
                            let Options {
                                watch,
                                ignore,
                                ..
                            } = &folder.options;
                            if ignore.contains(&parent) {
                                continue;
                            }
                            if *watch && path.matches_filters(&rule.filters) {
                                rule.actions.run(path);
                                break 'rules;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct Daemon;

impl Daemon {
    pub fn replace() -> Result<()> {
        let path = UserConfig::path();
        let lock_file = LockFile::new()?;
        match lock_file.get_process_by_path(&path) {
            Some(pid) => {
                {
                    // force sys to go out of scope before watch() is run
                    let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
                    sys.get_process(pid).unwrap().kill(Signal::Kill);
                }
                watch()
            }
            None => {
                // there is no running process
                if path == UserConfig::default_path() {
                    println!(
                        "{}",
                        "No instance was found running with the default configuration.".bold()
                    );
                } else {
                    println!(
                        "{} ({})",
                        "No instance was found running with the desired configuration".bold(),
                        path.display().to_string().underline()
                    );
                };
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Would you like to start a new instance?")
                    .interact()
                    .unwrap();

                if confirm {
                    watch()
                } else {
                    Ok(())
                }
            }
        }
    }
}
