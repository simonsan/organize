#[cfg(test)]
mod tests {
    use crate::lock_file::LockFile;
    use clap::crate_name;
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
        Command::new(crate_name!())
            .arg("stop")
            .spawn()
            .unwrap_or_else(|_| panic!("could not run {} stop", crate_name!()))
            .wait()
            .expect("command wasn't running");
        Command::new(crate_name!())
            .args(&["watch", "--daemon"])
            .spawn()
            .unwrap_or_else(|_| panic!("could not run '{} watch'", crate_name!()))
            .wait()
            .unwrap();
        thread::sleep(time::Duration::from_secs(1));
        Command::new(crate_name!())
            .arg("stop")
            .spawn()
            .unwrap_or_else(|_| panic!("could not run {} stop", crate_name!()))
            .wait()
            .expect("command wasn't running");
        let lock_file = LockFile::new()?;
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
