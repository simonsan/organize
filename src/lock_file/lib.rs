#[cfg(test)]
mod tests {
    use crate::{
        lock_file::LockFile,
        PROJECT_NAME,
    };
    use std::{
        fs,
        io::{
            Error,
            ErrorKind,
        },
        process::Command,
        thread,
        time,
    };

    #[test]
    fn clear_dead_processes() -> Result<(), Error> {
        Command::new(PROJECT_NAME)
            .arg("stop")
            .spawn()
            .unwrap_or_else(|_| panic!("could not run {} stop", PROJECT_NAME))
            .wait()
            .expect("command wasn't running");
        Command::new(PROJECT_NAME)
            .args(&["watch", "--daemon"])
            .spawn()
            .unwrap_or_else(|_| panic!("could not run '{} watch'", PROJECT_NAME))
            .wait()
            .unwrap();
        thread::sleep(time::Duration::from_secs(1));
        Command::new(PROJECT_NAME)
            .arg("stop")
            .spawn()
            .unwrap_or_else(|_| panic!("could not run {} stop", PROJECT_NAME))
            .wait()
            .expect("command wasn't running");
        let lock_file = LockFile::new();
        lock_file.clear_dead_processes().expect("couldnt write lockfile");
        let content = fs::read_to_string(&lock_file.path).expect("couldnt read lockfile");
        if content.is_empty() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "processes are not being cleared from lockfile properly",
            ))
        }
    }
}
