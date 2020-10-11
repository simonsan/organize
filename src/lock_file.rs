use std::{
    env::temp_dir,
    fs,
    fs::File,
    io::{
        Error,
        Read,
    },
    path::PathBuf,
};

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

    pub fn write_pid(&self, pid: i32) -> Result<(), Error> {
        fs::write(&self.path, format!("{}", &pid))
    }

    pub fn read_pid(&self) -> Result<i32, Error> {
        let content = fs::read_to_string(&self.path)?;
        let pid = content.parse::<i32>().unwrap();
        Ok(pid)
    }
}
