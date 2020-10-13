use std::{
    env::temp_dir,
    fs,
    io::Error,
    path::PathBuf,
};
use sysinfo::Pid;

#[derive(Default)]
pub struct LockFile {
    path: PathBuf,
}

impl LockFile {
    pub fn new() -> Self {
        LockFile {
            path: temp_dir().join("organizer.lock"),
        }
    }

    pub fn write_pid(self, pid: Pid) -> Result<(), Error> {
        fs::write(&self.path, format!("{}", &pid))
    }

    pub fn read_pid(self) -> Result<Pid, Error> {
        let content = fs::read_to_string(&self.path)?;
        let pid = content.parse::<Pid>().unwrap();
        Ok(pid)
    }
}
