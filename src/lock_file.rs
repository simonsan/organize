use std::{
    env::temp_dir,
    fs,
    fs::File,
    io::Error,
    path::PathBuf,
};

pub struct LockFile {
    path: PathBuf,
}

impl LockFile {
    pub fn new() -> Self {
        let mut path = temp_dir();
        path.push("organizer.lock");
        Self {
            path,
        }
    }

    pub fn create(&self) -> Result<(), Error> {
        File::create(&self.path)?;
        Ok(())
    }

    pub fn write_pid(self, pid: i32) -> Result<(), Error> {
        if !self.path.exists() {
            self.create()?;
        }
        fs::write(self.path, format!("{}", pid))?;
        Ok(())
    }

    pub fn get_pid(&self) -> Result<i32, Error> {
        assert!(self.path.exists());
        Ok(fs::read_to_string(&self.path)
            .unwrap()
            .parse::<i32>()
            .expect("could not parse PID"))
    }
}
