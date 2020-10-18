use std::{
    io::Error,
    process,
    sync::mpsc::{
        channel,
        Receiver,
    },
};

use colored::Colorize;
use notify::{
    op,
    raw_watcher,
    RawEvent,
    RecommendedWatcher,
    RecursiveMode,
    Watcher as OtherWatcher,
};

use crate::{
    lock_file::LockFile,
    path::MatchesFilters,
    subcommands::{
        run::run,
        stop::stop,
        watch::daemon::Daemon,
    },
    user_config::{
        rules::folder::Options,
        UserConfig,
    },
};
use clap::ArgMatches;
use dialoguer::{
    theme::ColorfulTheme,
    Confirm,
    Select,
};

pub mod daemon;

pub fn watch(args: &ArgMatches) -> Result<(), Error> {
    let lock_file = LockFile::new()?;
    let path = UserConfig::path(args);

    // REPLACE
    if args.subcommand().unwrap().1.is_present("replace") {
        let process = lock_file.find_process_by_path(&path);
        return match process {
            Some((pid, _)) => {
                let mut daemon = Daemon::new(Some(pid));
                daemon.restart(&path);
                Ok(())
            }
            None => {
                // there is no running process
                if path == UserConfig::default_path() {
                    println!("No instance was found running with the default set of rules.");
                } else {
                    println!(
                        "No instance was found running with the set of rules defined in {}",
                        path.display()
                    );
                }
                let prompt = "Would you like to start a new instance?";
                let confirm = Confirm::new().with_prompt(prompt).interact().unwrap();
                if confirm {
                    let mut daemon = Daemon::new(None);
                    daemon.start(&path);
                }
                Ok(())
            }
        };
    } else {
        // DAEMON

        if args.subcommand().unwrap().1.is_present("daemon") {
            let processes = lock_file.get_running_watchers();
            if !processes.is_empty() {
                println!("{}", "The following configurations were found running:".bold());
                for (_, path) in processes {
                    println!(" - {}", path.display().to_string().as_str().underline())
                }
                println!();
                let options = ["Stop instance", "Run anyway", "Do nothing"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    // .clear(true)
                    .with_prompt("Select an option:")
                    .items(&options)
                    .default(0)
                    .interact()
                    .unwrap();

                match selection {
                    0 => {
                        stop()?;
                    }
                    1 => {
                        let mut daemon = Daemon::new(None);
                        daemon.start(&path);
                    }
                    _ => {}
                }
            } else {
                let mut daemon = Daemon::new(None);
                daemon.start(&path);
            }
        // NO ARGS
        } else {
            let process = lock_file.find_process_by_pid(process::id() as i32);
            if process.is_some() {
                // if the pid has already been registered, that means that `organize` was run with the --daemon option
                // and the pid has already been registered
                run(args)?;
                let mut watcher = Watcher::new();
                watcher.run(args)?;
            } else {
                let options = ["Stop instance", "Run anyway", "Do nothing"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .clear(true)
                    .with_prompt("An existing instance was found:")
                    .items(&options)
                    .default(0)
                    .interact()
                    .unwrap();

                match selection {
                    0 => {
                        stop()?;
                    }
                    1 => {
                        run(args)?;
                        lock_file.append(process::id() as i32, &path)?;
                        let mut watcher = Watcher::new();
                        watcher.run(args)?;
                    }
                    _ => return Ok(()),
                }
            }
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

    pub fn run(&mut self, args: &ArgMatches) -> Result<(), Error> {
        let config = UserConfig::new(args)?;
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
                                rule.actions.run(path, true)?;
                                break 'rules;
                            }
                        }
                    }
                }
            }
        }
    }
}
