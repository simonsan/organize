use crate::{lock_file::LockFile, subcommands::watch::Daemon, user_config::UserConfig};
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use std::io::Error;
use sysinfo::{ProcessExt, RefreshKind, Signal, System, SystemExt};

pub fn stop() -> Result<(), Error> {
    let lock_file = LockFile::new()?;
    let watchers = lock_file.get_running_watchers();
    let pids = watchers.iter().map(|(pid, _)| pid).collect::<Vec<_>>();
    let paths = watchers.iter().map(|(_, path)| path.display()).collect::<Vec<_>>();

    if watchers.is_empty() {
        println!("No instance was found running.");
        let prompt = "Would you like to start a new daemon with the default configuration?";
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .unwrap();
        if confirm {
            Daemon::start(&UserConfig::default_path());
        }
    } else {
        let sys = System::new_with_specifics(RefreshKind::with_processes(RefreshKind::new()));
        if watchers.len() == 1 {
            sys.get_process(**pids.first().unwrap()).unwrap().kill(Signal::Kill);
        } else {
            let prompt = "Press SpaceBar to select one or more options and press Enter to stop them:";
            let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .items(&paths[..])
                .interact()
                .unwrap();
            for selection in selections {
                sys.get_process(**pids.get(selection).unwrap())
                    .unwrap()
                    .kill(Signal::Kill);
            }
        }
    }
    Ok(())
}
