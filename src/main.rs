pub mod cli;
pub mod configuration;
pub mod file;
pub mod lock_file;
pub mod subcommands;

use crate::{
    cli::Cli,
    lock_file::LockFile,
    subcommands::edit::UserConfig,
};
use colored::Colorize;
use std::{
    env,
    env::temp_dir,
    fs,
    fs::File,
    io::{
        Error,
        Read,
        Write,
    },
    path::PathBuf,
    process::Command,
};
use sysinfo::{
    Process,
    ProcessExt,
    Signal,
    System,
    SystemExt,
};

// TODO remove panics
fn main() -> Result<(), Error> {
    let cli = Cli::default();
    let config = UserConfig::new(&cli);
    match config {
        Ok(config) => cli.run(config).unwrap(),
        Err(e) => {
            let description = e.to_string();
            print!("{}", "ERROR: ".red());
            println!("{}", description);
        }
    }
    Ok(())
}

fn start_daemon() -> Result<(), Error> {
    let mut args = env::args();
    let command = args.next().unwrap(); // must've been started through a command
    let args: Vec<_> = args.filter(|arg| arg != "--daemon" && arg != "--replace").collect();
    let lock_file = LockFile::new();
    let pid = Command::new(command)
        .args(&args)
        .spawn()
        .expect("couldn't start daemon")
        .id() as i32;
    lock_file.write_pid(pid)?;
    println!("[1] {}", pid);
    Ok(())
}

pub fn kill_daemon() -> Result<(), Error> {
    let lock_file = LockFile::new();
    let pid = lock_file.get_pid()?;
    let sys = System::new_all();
    sys.get_processes()
        .get(&pid)
        .expect("could not find running instance's PID")
        .kill(Signal::Kill);
    Ok(())
}
