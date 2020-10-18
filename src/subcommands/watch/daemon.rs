use std::{
    env,
    process::Command,
};

use crate::{
    lock_file::LockFile,
    user_config::UserConfig,
};
use clap::ArgMatches;
use std::{
    convert::TryInto,
    path::Path,
};
use sysinfo::{
    Pid,
    ProcessExt,
    RefreshKind,
    Signal,
    System,
    SystemExt,
};

// TODO review fields are method args
#[derive(Clone, Debug)]
pub struct Daemon {
    pid: Option<Pid>,
}

impl Daemon {
    pub fn new(pid: Option<Pid>) -> Self {
        Daemon {
            pid,
        }
    }

    pub fn start(&mut self, path: &Path) {
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
        // self.pid = Some(pid);
        lock_file.append(pid.try_into().unwrap(), path).unwrap();
    }

    pub fn kill(&mut self) {
        #[cfg(debug_assertions)]
        assert!(self.pid.is_some());

        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        sys.get_process(self.pid.unwrap() as i32).unwrap().kill(Signal::Kill);
        self.pid = None;
    }

    pub fn restart(&mut self, path: &Path) {
        self.kill();
        self.start(path);
    }

    pub fn is_running(&self) -> bool {
        #[cfg(debug_assertions)]
        assert!(self.pid.is_some());

        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        let process = sys.get_process(self.pid.unwrap());
        process.is_some()
    }
}
