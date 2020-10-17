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
    pid: Option<Pid>,
}

impl Daemon {
    pub fn new(pid: Option<Pid>) -> Self {
        Daemon {
            pid,
        }
    }

    pub fn start(&mut self) {
        let mut args = env::args();
        let command = args.next().unwrap(); // must've been started through a command
        let args: Vec<_> = args
            .filter(|arg| arg != "--daemon" && arg != "--replace" && arg != "stop")
            .collect();
        let pid = Command::new(command)
            .args(&args)
            .spawn()
            .expect("couldn't start daemon")
            .id() as i32;
        println!("[1] {}", pid);
        self.pid = Some(pid);
    }

    pub fn kill(&mut self) {
        #[cfg(debug_assertions)]
        assert!(self.pid.is_some());

        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        sys.get_process(self.pid.unwrap() as i32).unwrap().kill(Signal::Kill);
        self.pid = None;
    }

    pub fn restart(&mut self) {
        self.kill();
        self.start();
    }

    pub fn is_running(&self) -> bool {
        #[cfg(debug_assertions)]
        assert!(self.pid.is_some());

        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        let process = sys.get_process(self.pid.unwrap());
        process.is_some()
    }
}
