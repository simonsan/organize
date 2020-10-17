use crate::{
    commands::watch::daemon::Daemon,
    lock_file::LockFile,
};
use dialoguer::{
    theme::ColorfulTheme,
    Confirm,
    MultiSelect,
};
use std::{
    io::Error,
    process,
};

pub fn stop() -> Result<(), Error> {
    let lock_file = LockFile::new();
    lock_file.clear_dead_processes()?;
    let watchers = lock_file.get_running_watchers();
    let pids = watchers.iter().map(|(pid, _)| pid).collect::<Vec<_>>();
    let paths = watchers.iter().map(|(_, path)| path.display()).collect::<Vec<_>>();

    if watchers.is_empty() {
        println!("No instance was found running.");
        let prompt = "Would you like to start a new daemon with the default configuration?";
        let confirm = Confirm::new().with_prompt(prompt).interact();
        if confirm.is_ok() && confirm.unwrap() {
            let daemon = Daemon::new(process::id() as i32);
            daemon.start();
        }
    } else if watchers.len() == 1 {
        let daemon = Daemon::new(**pids.first().unwrap());
        daemon.kill();
    } else {
        let prompt = "Press Spacebar to select one or more options and press Enter to stop them:";
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&paths[..])
            .interact()
            .unwrap();
        for selection in selections {
            let daemon = Daemon::new(**pids.get(selection).unwrap());
            daemon.kill();
        }
    }
    Ok(())
}
