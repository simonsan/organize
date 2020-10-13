use std::{
    io::{
        Error,
        ErrorKind,
    },
    path::PathBuf,
    process::Command,
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
    match std::env::var("EDITOR") {
        Ok(editor) => {
            let mut editor = Command::new(editor);
            editor
                .arg(path.to_str().unwrap())
                .spawn()
                .expect("ERROR: failed to run editor")
                .wait()
                .expect("ERROR: command was not running");
            Ok(())
        }
        Err(_) => Err(Error::new(ErrorKind::NotFound, crate::utils::prompt_editor_env_var())),
    }
}
