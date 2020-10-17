use std::{
    convert::TryInto,
    io::Error,
    process,
    sync::mpsc::{
        channel,
        Receiver,
    },
};

use notify::{
    op,
    raw_watcher,
    RawEvent,
    RecommendedWatcher,
    RecursiveMode,
    Watcher as OtherWatcher,
};

use crate::{
    commands::{
        run::run,
        stop::stop,
        watch::daemon::Daemon,
    },
    file::File,
    lock_file::LockFile,
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
    if args.is_present("replace") {
        let process = lock_file.find_process_by_path(&path);
        return match process {
            Some((pid, _)) => {
                let mut daemon = Daemon::new(Some(pid));
                daemon.restart();
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
                    daemon.start();
                }
                Ok(())
            }
        };
    } else {
        let processes = lock_file.get_running_watchers();
        for (_, process_path) in processes {
            if path == process_path {
                let options = ["Stop instance", "Run anyway", "Do nothing"];
                let selection = Select::with_theme(&ColorfulTheme::default())
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
                        let mut daemon = Daemon::new(None);
                        daemon.start();
                    }
                    _ => {}
                }
            }
        }

        // DAEMON
        if args.is_present("daemon") {
            let mut daemon = Daemon::new(None);
            daemon.start();
        // NO ARGS
        } else {
            run(args)?;
            let mut watcher = Watcher::new();
            watcher.run(args, lock_file)?;
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

    pub fn run(&mut self, args: &ArgMatches, lock_file: LockFile) -> Result<(), Error> {
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

        // REGISTER PID
        let pid = process::id();
        let path = UserConfig::path(args);
        lock_file.append(pid.try_into().unwrap(), &path).unwrap();
        std::mem::drop(lock_file);

        // PROCESS SIGNALS
        let path2rules = config.to_map();
        loop {
            if let Ok(RawEvent {
                path: Some(abs_path),
                op: Ok(op),
                ..
            }) = self.receiver.recv()
            {
                if let op::CREATE = op {
                    let mut file = File::from(abs_path.as_path());
                    if file.path.is_file() {
                        let parent_dir = file.path.parent().unwrap().to_path_buf();
                        let values = path2rules.get(&parent_dir).unwrap().to_owned();
                        'rules: for (rule, i) in values {
                            let folder = rule.folders.get(i).unwrap();
                            let Options {
                                watch,
                                ignore,
                                ..
                            } = &folder.options;
                            if ignore.contains(&parent_dir) {
                                continue;
                            }
                            if *watch && file.matches_filters(&rule.filters) {
                                rule.actions.run(&mut file, true).unwrap();
                                break 'rules;
                            }
                        }
                    }
                }
            }
        }
    }
}
