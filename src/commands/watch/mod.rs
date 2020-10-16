use std::{
    convert::TryInto,
    io::{
        Error,
        ErrorKind,
    },
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
    cli::Cli,
    commands::{
        run::run,
        watch::daemon::Daemon,
    },
    file::File,
    lock_file::LockFile,
    user_config::{
        rules::folder::Options,
        UserConfig,
    },
    PROJECT_NAME,
};

pub mod daemon;

pub fn watch(cli: Cli) -> Result<(), Error> {
    let lock_file = LockFile::new();
    let path = UserConfig::path(&cli);

    // REPLACE
    if cli.args.is_present("replace") {
        let process = lock_file.find_process_by_path(&path);
        return match process {
            Some((pid, _)) => {
                let daemon = Daemon::new(pid);
                daemon.restart();
                Ok(())
            }
            None => {
                // there is no running process
                if path == UserConfig::default_path() {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "no running instance was found for the default configuration. \n\
                        Run `{} watch` or '{} watch --daemon' to start a new instance with the default config\n\
                        You can also run '{} watch --daemon --with-config <path>'",
                            PROJECT_NAME, PROJECT_NAME, PROJECT_NAME
                        ),
                    ))
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!("no running instance was found for the desired configuration. \n\
                        Run `{} watch` or '{} watch --daemon --with-config {}' to start a new instance with this configuration",
                                PROJECT_NAME, PROJECT_NAME, path.display()),
                    ))
                }
            }
        };
    } else {
        let processes = lock_file.get_running_watchers();
        for (_, process_path) in processes {
            if path == process_path {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "a running instance already exists with the desired configuration. \
                        Use `{} stop --with-config {}` to stop this instance, '{} stop' to stop all instances \
                        or `{} watch --daemon --replace --with-config {}` to restart the daemon",
                        PROJECT_NAME,
                        &path.display(),
                        PROJECT_NAME,
                        PROJECT_NAME,
                        &path.display()
                    ),
                ));
            } else if path != process_path && !cli.args.is_present("allow_multiple_instances") {
                return Err(
                    Error::new(
                        ErrorKind::Other,
                        format!("an instance is already running with config {} \n\
                        make sure that new the configuration doesn't overlap with the existing one and then run again with --allow-multiple-instances", path.canonicalize()?.display()),
                    )
                );
            }
        }

        // DAEMON
        if cli.args.is_present("daemon") {
            let daemon = Daemon::new(process::id() as i32);
            daemon.start();
        // NO ARGS
        } else {
            let config = UserConfig::new(&cli)?;
            let mut watcher = Watcher::new();
            run(&config, false)?;
            watcher.watch(cli, config)?;
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

    pub fn watch(&mut self, cli: Cli, config: UserConfig) -> Result<(), Error> {
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
        let lock_file = LockFile::new();
        let path = UserConfig::path(&cli);
        lock_file.append(pid.try_into().unwrap(), &path).unwrap();
        std::mem::drop(lock_file);
        std::mem::drop(cli);

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
