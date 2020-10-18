use std::{
    env,
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
use std::{
    convert::TryInto,
    path::Path,
    process::Command,
};
use sysinfo::{
    ProcessExt,
    RefreshKind,
    Signal,
    System,
    SystemExt,
};

pub fn watch(args: &ArgMatches) -> Result<(), Error> {
    let lock_file = LockFile::new()?;
    let path = UserConfig::path(args);

    // REPLACE
    if args.subcommand().unwrap().1.is_present("replace") {
        Daemon::replace(args)?;
    } else {
        // DAEMON

        if args.subcommand().unwrap().1.is_present("daemon") {
            let processes = lock_file.get_running_watchers();
            if processes.is_empty() {
                Daemon::start(&path);
            } else {
                println!("{}", "The following configurations were found running:".bold());
                for (_, path) in processes {
                    println!(
                        " {} {}",
                        "·".bright_black(),
                        path.display().to_string().as_str().underline()
                    )
                }
                println!();
                let options = ["Stop instance and run", "Run anyway", "Do nothing"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select an option:")
                    .items(&options)
                    .default(0)
                    .interact()
                    .unwrap();

                match selection {
                    0 => {
                        stop()?;
                        Daemon::start(&path);
                    }
                    1 => Daemon::start(&path),
                    _ => {}
                }
            }
        // NO ARGS
        } else if lock_file.get_running_watchers().is_empty() {
            run(args)?;
            lock_file.append(process::id() as i32, &path)?;
            std::mem::drop(path);
            std::mem::drop(lock_file);
            let mut watcher = Watcher::new();
            watcher.run(args)?;
        } else if lock_file.find_process_by_pid(process::id() as i32).is_some() {
            // if the pid has already been registered, that means that `organize` was run with the --daemon option
            // and we don't need to prompt the user
            run(args)?;
            let mut watcher = Watcher::new();
            watcher.run(args)?;
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
                    run(args)?;
                    lock_file.append(process::id() as i32, &path)?;
                    std::mem::drop(lock_file);
                    std::mem::drop(path);
                    let mut watcher = Watcher::new();
                    watcher.run(args)?;
                }
                _ => {}
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
        let config = UserConfig::new(&args)?;
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

pub(crate) struct Daemon;

impl Daemon {
    pub fn start(path: &Path) {
        let mut args = env::args();
        let command = args.next().unwrap(); // must've been started through a command
        let mut args: Vec<_> = args
            .filter(|arg| arg != "--daemon" && arg != "--replace" && arg != "stop")
            .collect();
        if args.is_empty() {
            // if called from the stop command, `args` will be empty
            // so we must push `watch` to be able to run the daemon
            args.push("watch".into());
        }
        let lock_file = LockFile::new().unwrap();
        let pid = Command::new(command)
            .args(&args)
            .spawn()
            .expect("couldn't start daemon")
            .id() as i32;
        lock_file.append(pid.try_into().unwrap(), path).unwrap();
    }

    pub fn replace(args: &ArgMatches) -> Result<(), Error> {
        let path = UserConfig::path(args);
        let lock_file = LockFile::new()?;
        let process = lock_file.find_process_by_path(&path);
        match process {
            Some((pid, _)) => {
                let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
                sys.get_process(pid).unwrap().kill(Signal::Kill);
                Daemon::start(&path);
                Ok(())
            }
            None => {
                // there is no running process
                if path == UserConfig::default_path() {
                    println!("No instance was found running with the default configuration.");
                } else {
                    println!(
                        "No instance was found running with the configuration {}",
                        path.display()
                    );
                }
                let prompt = "Would you like to start a new instance?";
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(prompt)
                    .interact()
                    .unwrap();
                if confirm {
                    Daemon::start(&path);
                }
                Ok(())
            }
        }
    }
}
