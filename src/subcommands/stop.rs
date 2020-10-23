use crate::{lock_file::LockFile, user_config::UserConfig, MATCHES};
use clap::crate_name;
use std::io::Result;
use sysinfo::{ProcessExt, RefreshKind, Signal, System, SystemExt};

pub fn stop() -> Result<()> {
    let lock_file = LockFile::new()?;
    let watchers = lock_file.get_running_watchers();

    if watchers.is_empty() {
        println!("No instance was found running.");
    } else {
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        if MATCHES.subcommand().unwrap().1.is_present("all") {
            for process in sys.get_process_by_name(crate_name!()) {
                process.kill(Signal::Kill);
            }
        } else {
            let path = UserConfig::path();
            match lock_file.get_process_by_path(&path) {
                Some(pid) => {
                    sys.get_process(pid).unwrap().kill(Signal::Kill);
                }
                None => {
                    println!("No instance was found running with configuration: {}", path.display());
                }
            }
        }
    }
    Ok(())
}
