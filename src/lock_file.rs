use std::{
    env::temp_dir,
    fs,
    io::{
        Error,
        prelude::*,
    },
    path::{
        Path,
        PathBuf,
    },
};
use std::fs::OpenOptions;

use sysinfo::Pid;

pub struct LockFile {
    path: PathBuf,
    sep: String,
}

impl LockFile {
    pub fn new() -> Self {
        LockFile {
            path: temp_dir().join("organizer.lock"),
            sep: "---".to_string(),
        }
    }

    pub fn write(self, pid: Pid, config: &Path) -> Result<(), Error> {
        fs::write(&self.path, format!("{}\n{}\n{}", pid, config.display(), self.sep))
    }

    pub fn append(self, pid: Pid, config: &Path) -> Result<(), Error> {
        let mut f = OpenOptions::new().append(true).open(config)?;
        writeln!(f, "{}\n{}\n{}", pid, config.display(), self.sep)
    }

    pub fn get_sections(&self) -> Result<Vec<(Pid, PathBuf)>, Error> {
        let content = fs::read_to_string(&self.path)?;
        let content = content.split(&self.sep);
        let mut sections = Vec::new();
        for section in content.into_iter() {
            let section: Vec<String> = section.lines().map(|x| x.to_string()).collect();
            let pid = section.get(0).unwrap().parse().unwrap();
            let path = section.get(1).unwrap().parse().unwrap();
            sections.push((pid, path))
        }
        Ok(sections)
    }
}
