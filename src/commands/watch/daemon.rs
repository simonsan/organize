use std::{
    env,
    process::Command,
};

use sysinfo::{
    Pid,
    ProcessExt,
    RefreshKind,
    Signal,
    System,
    SystemExt,
};

use crate::cli::Cli;
// TODO review fields are method args
#[derive(Clone, Debug)]
pub struct Daemon<'a> {
    cli: &'a Cli,
    pid: Option<Pid>,
}

impl<'a> Daemon<'a> {
    pub fn new(cli: &'a Cli, pid: Option<Pid>) -> Self {
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
        assert!(self.pid.is_some());
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        sys.get_process(self.pid.unwrap() as i32).unwrap().kill(Signal::Kill);
    }

    pub fn restart(&self) {
        self.kill();
        self.start();
    }

    pub fn is_running(&self) -> bool {
        assert!(self.pid.is_some());
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        let process = sys.get_process(self.pid.unwrap());
        process.is_some()
    }
}
