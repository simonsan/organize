use crate::{
    config_path,
    configuration::{
        rules::Rule,
        temporary::rules::TemporaryRules,
    },
};
use clap::ArgMatches;
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
        Err(_) => Err(Error::new(ErrorKind::NotFound, utils::prompt_editor_env_var())),
    }
}

/// Represents the user's configuration file
/// ### Fields
/// * `path`: the path the user's config, either the default one or some other passed with the --with-config argument
/// * `rules`: a list of parsed rules defined by the user
pub struct UserConfig {
    pub rules: Vec<Rule>,
}

pub struct Rules(Vec<Rule>);

impl UserConfig {
    /// Creates a new UserConfig instance.
    /// It parses the configuration file
    /// and fills missing fields with either the defaults, in the case of global options,
    /// or with the global options, in the case of folder-level options.
    /// If the config file does not exist, it is created.
    /// ### Errors
    /// This constructor fails in the following cases:
    /// - The configuration file does not exist
    pub fn new(args: &ArgMatches) -> Result<Self, Error> {
        let path = match args.value_of("with_config") {
            Some(path) => PathBuf::from(path),
            None => config_path(),
        };

        if !path.exists() {
            utils::create_config_file(&path)?;
        }

        let temp_rules = TemporaryRules::new(&path)?;
        let mut rules = Vec::new();
        for temp_rule in temp_rules.0 {
            rules.push(temp_rule.unwrap())
        }

        Ok(UserConfig {
            rules,
        })
    }

    /// Validates the user's config.
    /// ### Errors
    /// This function returns an error in the following cases:
    /// - An empty string was provided as the path to a folder
    /// - The path supplied to a folder does not exist
    /// - The path supplied to a folder is not a directory
    /// - No path was supplied to a folder
    pub fn validate(self) -> Result<Self, Error> {
        for (i, rule) in self.rules.iter().enumerate() {
            rule.actions.check_conflicting_actions()?;
            for (j, folder) in rule.folders.iter().enumerate() {
                if folder.path.display().to_string().eq("") {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "path defined in field 'path' cannot be an empty value (rule {}, folder {})",
                            j, i
                        ),
                    ));
                } else if !folder.path.exists() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("path defined in field 'path' does not exist (rule {}, folder {})", j, i),
                    ));
                } else if !folder.path.is_dir() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "path defined in field 'path' is not a directory (rule {}, folder {})",
                            j, i
                        ),
                    ));
                }
            }
        }
        Ok(self)
    }
}

pub mod utils {
    use crate::PROJECT_NAME;
    use clap::load_yaml;
    use std::{
        io::{
            Error,
            ErrorKind,
        },
        path::Path,
    };
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
                let default_config = load_yaml!("../../examples/example_config.yml");
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

    pub(in crate::subcommands::edit) fn prompt_editor_env_var() -> String {
        let platform = std::env::consts::OS;
        if platform == "linux" || platform == "macos" {
            format!("{} could not find an $EDITOR environment variable or it's not properly set.\nIn your .bashrc (or .zshrc), set 'export EDITOR=$(which <your-favorite-editor-name>) or \
                run {} as 'EDITOR=$(which <your-favorite-editor-name>) {} config'", PROJECT_NAME, PROJECT_NAME, PROJECT_NAME)
        } else if platform == "windows" {
            format!(
                "{} could not find an EDITOR environment variable or it's not properly set",
                PROJECT_NAME
            )
        } else {
            format!("{} platform not supported", platform)
        }
    }
}
