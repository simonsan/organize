use std::{
    env,
    io::{
        Error,
        ErrorKind,
    },
    process::Command,
};
use std::path::Path;

use sysinfo::{
    Pid,
    ProcessExt,
    RefreshKind,
    Signal,
    System,
    SystemExt,
};

use crate::{
    cli::{
        config_path,
        Cli,
    },
};

#[derive(Clone, Debug)]
pub struct Daemon<'a> {
    cli: &'a Cli,
    pid: Pid,
}

impl<'a> Daemon<'a> {
    pub fn new(cli: &'a Cli, pid: Pid) -> Self {
        Daemon {
            cli,
            pid,
        }
    }

    pub fn start(&self) {
        let mut args = env::args();
        let command = args.next().unwrap(); // must've been started through a command
        let args: Vec<_> = args.filter(|arg| arg != "--daemon" && arg != "--replace").collect();
        let pid = Command::new(command)
            .args(&args)
            .spawn()
            .expect("couldn't start daemon")
            .id() as i32;
        println!("[1] {}", pid);
    }

    pub fn kill(&self) {
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        sys.get_process(self.pid as i32).unwrap().kill(Signal::Kill);
    }

    pub fn restart(&self) {
        self.kill();
        self.start();
    }

    pub fn is_running(&self) -> bool {
        let process = sys.get_process(&self.pid);
        process.is_some()
    }
}
