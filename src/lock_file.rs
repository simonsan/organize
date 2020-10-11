use std::{
    env::temp_dir,
    fs,
    fs::File,
    io::Error,
    path::PathBuf,
};

#[derive(Default)]
pub struct LockFile {
    pub path: PathBuf,
}

impl LockFile {
    pub fn new() -> Self {
        let path = temp_dir().join("organizer.lock");
        Self {
            path,
        }
    }

    pub fn delete(&self) -> Result<(), Error> {
        Ok(fs::remove_file(&self.path)?)
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
        let f = fs::read_to_string(&self.path)?
            .parse::<i32>()
            .expect("could not parse PID");
        Ok(f)
    }
}
