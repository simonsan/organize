use clap::load_yaml;
use std::io::{Error, ErrorKind};
use std::path::Path;
use yaml_rust::YamlEmitter;

pub fn error(kind: ErrorKind, msg: &str) -> Error {
    Error::new(kind, msg)
}

pub fn create_config_file(path: &Path) -> Result<(), Error> {
    // safe unwrap, dir is created at $HOME or $UserProfile%,
    // so it exists and the user must have permissions
    if path.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "{} already exists in this directory",
                path.file_name().unwrap().to_str().unwrap()
            ),
        ));
    }
    match path.parent() {
        Some(parent) => {
            if !parent.exists() {
                std::fs::create_dir_all(path.parent().unwrap())?;
            }
            let default_config = load_yaml!("../../../examples/example_config.yml");
            let mut output = String::new();
            let mut emitter = YamlEmitter::new(&mut output);
            emitter
                .dump(default_config)
                .expect("ERROR: could not create starter config");
            std::fs::write(path, output)?;
        }
        None => panic!("home directory's parent folder should be defined"),
    }
    Ok(())
}

pub(in crate::subcommands::config) fn prompt_editor_env_var() -> String {
    let platform = std::env::consts::OS;
    if platform == "linux" || platform == "macos" {
        String::from(
			"d-organizer could not find an $EDITOR environment variable or it's not properly set.\nIn your .bashrc (or .zshrc), set 'export EDITOR=$(which <your-favorite-editor-name>) or \
                    run d-organizer as 'EDITOR=$(which <your-favorite-editor-name>) d-organizer config'",
		)
    } else if platform == "windows" {
        String::from("d-organizer could not find an EDITOR environment variable or it's not properly set")
    } else {
        format!("{} platform not supported", platform)
    }
}
