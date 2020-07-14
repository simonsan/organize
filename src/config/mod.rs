use crate::cli::{Cli, SubCommands};
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;
use yaml_rust::{Yaml, YamlEmitter};

pub mod actions;
mod filters;

pub struct UserConfig<'a> {
    args: &'a Cli<'a>,
    path: &'a Path,
}

impl<'a> UserConfig<'a> {
    pub fn new(args: &'a Cli, path: &'a Path) -> Self {
        UserConfig {
            args,
            path,
        }
    }

    pub fn create_config_dir(&self) -> Result<&Self, Error> {
        assert!(!self.path.parent().unwrap().exists());
        std::fs::create_dir(self.path.parent().unwrap())?;
        Ok(self)
    }

    pub fn create_config_file(&self, default_config: &'a Yaml) -> Result<&Self, Error> {
        // safe unwrap, dir is created at $HOME or $UserProfile%,
        // so it exists and the user must have permissions
        assert!(!self.path.exists());
        let mut output = String::new();
        let mut emitter = YamlEmitter::new(&mut output);
        emitter
            .dump(default_config)
            .expect("ERROR: could not create starter config");
        std::fs::write(&self.path, output)?;
        Ok(self)
    }

    fn prompt_editor_env_var(&self) -> String {
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

    pub fn show_path(&self) {
        assert_eq!(self.args.subcommand.0, SubCommands::Config);
        println!("{}", self.path.to_str().unwrap());
    }

    pub fn edit_config(&self) -> Result<&Self, Error> {
        assert_eq!(self.args.subcommand.0, SubCommands::Config);
        match std::env::var("EDITOR") {
            Ok(editor) => {
                let mut editor = Command::new(editor);
                editor
                    .arg(self.path.to_str().unwrap())
                    .spawn()
                    .expect("ERROR: failed to run editor")
                    .wait()
                    .expect("ERROR: command was not running");
                Ok(self)
            }
            Err(_) => {
                let error_msg = self.prompt_editor_env_var();
                Err(Error::new(ErrorKind::NotFound, error_msg))
            }
        }
    }
}
