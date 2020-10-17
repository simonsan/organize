use crate::{
    lock_file::LockFile,
    subcommands::watch::daemon::Daemon,
};
use dialoguer::{
    theme::ColorfulTheme,
    Confirm,
    MultiSelect,
};
use std::io::Error;

pub fn stop() -> Result<(), Error> {
    let lock_file = LockFile::new()?;
    let watchers = lock_file.get_running_watchers();
    let pids = watchers.iter().map(|(pid, _)| pid).collect::<Vec<_>>();
    let paths = watchers.iter().map(|(_, path)| path.display()).collect::<Vec<_>>();

    if watchers.is_empty() {
        println!("No instance was found running.");
        let prompt = "Would you like to start a new daemon with the default configuration?";
        let confirm = Confirm::new().with_prompt(prompt).interact();
        if confirm.is_ok() && confirm.unwrap() {
            let mut daemon = Daemon::new(None);
            daemon.start();
        }
    } else if watchers.len() == 1 {
        let mut daemon = Daemon::new(Some(**pids.first().unwrap()));
        daemon.kill();
    } else {
        let prompt = "Press SpaceBar to select one or more options and press Enter to stop them:";
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&paths[..])
            .interact()
            .unwrap();
        for selection in selections {
            let mut daemon = Daemon::new(Some(**pids.get(selection).unwrap()));
            daemon.kill();
        }
    }
    Ok(())
}
