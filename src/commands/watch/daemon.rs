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

// TODO review fields are method args
#[derive(Clone, Debug)]
pub struct Daemon {
    pid: Pid,
}

impl Daemon {
    pub fn new(pid: Pid) -> Self {
        Daemon {
            pid,
        }
    }

    pub fn start(&self) {
        let mut args = env::args();
        let command = args.next().unwrap(); // must've been started through a command
        let mut args: Vec<_> = args
            .filter(|arg| arg != "--daemon" && arg != "--replace" && arg != "stop")
            .collect();
        if args.is_empty() {
            // allows to start daemon from the stop command
            args.push("watch".into());
        }
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
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        let process = sys.get_process(self.pid);
        process.is_some()
    }
}
