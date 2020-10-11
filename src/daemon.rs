use crate::lock_file::LockFile;
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
    Signal,
    System,
    SystemExt,
};

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
        Ok(pid)
    }

    pub fn kill(&self) -> Result<(), Error> {
        let lock_file = LockFile::new();
        let pid = lock_file.get_pid()?;
        let sys = System::new_all();
        sys.get_processes()
            .get(&pid)
            .expect("could not find running instance's PID")
            .kill(Signal::Kill);
        Ok(())
    }

    pub fn restart(&self) -> Result<i32, Error> {
        match self.kill() {
            Ok(_) => {
                let lock_file = LockFile::new();
                let pid = self.start()?;
                lock_file.write_pid(pid)?;
                Ok(pid)
            }
            Err(_) => Err(Error::new(
                ErrorKind::Other,
                "no running instance was found\nrun without --replace to start a new instance",
            )),
        }
    }
}
