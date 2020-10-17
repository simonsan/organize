#[cfg(test)]
mod tests {
    use crate::{
        lock_file::LockFile,
        subcommands::watch::daemon::Daemon,
        user_config::UserConfig,
    };
    use std::{
        convert::TryInto,
        fs,
        io::{
            Error,
            ErrorKind,
        },
    };
    use sysinfo::{
        RefreshKind,
        System,
        SystemExt,
    };

    fn stop() {
        let lock_file = LockFile::new().unwrap();
        let watchers = lock_file.get_running_watchers();
        for (pid, _) in watchers.iter() {
            let mut daemon = Daemon::new(Some(*pid));
            daemon.kill();
        }
    }

    fn simulate_watch() {
        let pid = 1000000000i32;
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        assert!(sys.get_process(pid).is_none());
        let lock_file = LockFile::new().unwrap();
        let path = UserConfig::default_path();
        lock_file.append(pid.try_into().unwrap(), &path).unwrap();
    }

    #[test]
    fn clear_dead_processes() -> Result<(), Error> {
        stop();
        simulate_watch();
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
