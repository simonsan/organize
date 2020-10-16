use std::{
    env,
    ffi::OsString,
    io::Error,
    path::PathBuf,
    process,
};

/// Launches an editor to modify the default config.
/// This function represents the `config` subcommand without any arguments.
/// ### Errors
/// This functions returns an error in the following cases:
/// - There is no $EDITOR environment variable.
/// ### Panics
/// This functions panics in the following cases:
/// - The $EDITOR env. variable was found but its process could not be started.
pub fn edit(path: PathBuf) -> Result<(), Error> {
    let editor = get_default_editor();
    process::Command::new(&editor).arg(path).spawn()?.wait()?;
    Ok(())
}

fn get_default_editor() -> OsString {
    if let Some(prog) = env::var_os("VISUAL") {
        return prog;
    }
    if let Some(prog) = env::var_os("EDITOR") {
        return prog;
    }
    if cfg!(windows) {
        "notepad.exe".into()
    } else {
        "vi".into()
    }
}
