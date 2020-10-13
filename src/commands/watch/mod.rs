pub mod daemon;

use crate::{
    cli::Cli,
    file::File,
    commands::{
        run::run,
        watch::daemon::Daemon,
    },
    user_config::{
        rules::{
            folder::Options,
            rule::Rule,
        },
        user_config::UserConfig,
    },
    utils::path2rules,
    PROJECT_NAME,
};
use notify::{
    op,
    raw_watcher,
    RawEvent,
    RecommendedWatcher,
    RecursiveMode,
    Watcher as OtherWatcher,
};
use std::{io::{
    Error,
    ErrorKind,
}, sync::mpsc::{
    channel,
    Receiver,
}, process};
use crate::lock_file::LockFile;
use std::convert::TryInto;

pub fn watch(cli: Cli, config: &UserConfig) -> Result<(), Error> {
    let daemon = Daemon::new();
    if cli.subcommand.1.is_present("replace") {
        daemon.restart()?;
    } else if !daemon.is_running().0 {
        if cli.subcommand.1.is_present("daemon") {
            daemon.start()?;
        } else {
            run(config.rules.to_owned(), false)?;
            let mut watcher = Watcher::new();
            watcher.watch(&config.rules);
        }
    } else {
        return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("a running instance already exists. Use `{} stop` to stop this instance or `{} watch --daemon --replace` to restart the daemon", PROJECT_NAME, PROJECT_NAME)
                )
            );
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

    pub fn watch(&mut self, rules: &[Rule]) {
        for rule in rules.iter() {
            for folder in rule.folders.iter() {
                let is_recursive = if folder.options.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                };
                self.watcher.watch(&folder.path, is_recursive).unwrap();
            }
        }
        // REGISTER THE PID
        let pid = process::id();
        let lock_file = LockFile::new();
        lock_file.write_pid(pid.try_into().unwrap()).unwrap();

        // PROCESS SIGNALS
        let path2rules = path2rules(&rules);
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