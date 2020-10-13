use crate::{
    lock_file::LockFile,
};
use std::{
    env,
    io::{
        Error,
        ErrorKind,
    },
    process::Command,
};
use sysinfo::{
    ProcessExt,
    RefreshKind,
    Signal,
    System,
    SystemExt,
    Pid
};

#[derive(Default, Clone, Debug)]
pub struct Daemon;

impl Daemon {
    pub fn new() -> Self {
        Daemon
    }

    pub fn start(&self) -> Result<Pid, Error> {
        let mut args = env::args();
        let command = args.next().unwrap(); // must've been started through a command
        let args: Vec<_> = args.filter(|arg| arg != "--daemon" && arg != "--replace").collect();
        let pid = Command::new(command)
            .args(&args)
            .spawn()
            .expect("couldn't start daemon")
            .id() as i32;
        println!("[1] {}", pid);
        Ok(pid)
    }

    pub fn kill(&self) -> Result<(), Error> {
        let (_, pid) = self.is_running();
        if let Some(pid) = pid {
            let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
            sys.get_process(pid as i32).unwrap().kill(Signal::Kill);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "no running instance was found"))
        }
    }

    pub fn restart(&self) -> Result<Pid, Error> {
        match self.kill() {
            Ok(_) => {
                let pid = self.start()?;
                Ok(pid)
            }
            Err(_) => Err(Error::new(
                ErrorKind::Other,
                "no running instance was found\nrun without --replace to start a new instance",
            )),
        }
    }

    pub fn is_running(&self) -> (bool, Option<Pid>) {
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        let lock_file = LockFile::new();
        let pid = lock_file.read_pid();
        if pid.is_err() {
            return (false, None);
        }
        let pid = pid.unwrap();
        let process = sys.get_process(pid.clone());
        (process.is_some(), Some(pid))
    }
}
