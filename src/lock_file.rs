use std::{
    env::temp_dir,
    fs,
    io::Error,
    path::PathBuf,
};
use std::path::Path;

use sysinfo::Pid;

pub struct LockFile<'a> {
    path: PathBuf,
    config: &'a Path,
}

impl<'a> LockFile<'a> {
    pub fn new(config: &'a Path) -> Self {
        LockFile {
            path: temp_dir().join("organizer.lock"),
            config,
        }
    }

    pub fn write(self, pid: Pid) -> Result<(), Error> {
        fs::write(&self.path, format!("{}\n{}", &pid, self.config.display()))
    }

    fn read(&self) -> Result<Vec<String>, Error> {
        let content = fs::read_to_string(&self.path)?;
        let lines: Vec<String> = content.lines().map(|x| x.to_string()).collect();
        Ok(lines)
    }

    pub fn get_pid_and_config(self) -> Result<(Pid, PathBuf), Error> {
        let lines = self.read()?;
        println!("{:?}", lines);
        Ok((lines.get(0).unwrap().parse::<i32>().unwrap(), PathBuf::from(lines.get(1).unwrap())))
    }
}
