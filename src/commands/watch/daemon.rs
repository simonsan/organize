use crate::{
    lock_file::LockFile,
    PROJECT_NAME,
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
};

#[derive(Default, Clone, Debug)]
pub struct Daemon;

impl Daemon {
    pub fn new() -> Self {
        Daemon
    }

    pub fn start(&self) -> Result<i32, Error> {
        let mut args = env::args();
        let command = args.next().unwrap(); // must've been started through a command
        let args: Vec<_> = args.filter(|arg| arg != "--daemon" && arg != "--replace").collect();
        let pid = Command::new(command)
            .args(&args)
            .spawn()
            .expect("couldn't start daemon")
            .id() as i32;
        println!("[1] {}", pid);
        let lock_file = LockFile::new();
        lock_file.write_pid(pid)?;
        Ok(pid)
    }

    pub fn kill(&self) -> Result<(), Error> {
        let pid = self.running_instance();
        if let Some(pid) = pid {
            let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
            sys.get_process(pid).unwrap().kill(Signal::Kill);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "no running instance was found"))
        }
    }

    pub fn restart(&self) -> Result<i32, Error> {
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

    pub fn is_runnable(&self) -> bool {
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        let lock_file = LockFile::new();
        let pid = lock_file.read_pid();
        if pid.is_err() {
            return true;
        }
        let process = sys.get_process(pid.unwrap());
        if process.is_none() {
            return true;
        }
        let processes = sys.get_process_by_name(PROJECT_NAME);
        processes.len() <= 1
    }

    fn running_instance(&self) -> Option<i32> {
        let lock_file = LockFile::new();
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        let pid = lock_file.read_pid();
        if pid.is_err() {
            return None;
        }
        let pid = pid.unwrap();
        let process = sys.get_process(pid);
        if process.is_some() {
            Some(pid)
        } else {
            None
        }
    }
}
