use std::{
    collections::HashMap,
    env,
    io,
    io::{
        Error,
        ErrorKind,
        Read,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};

use clap::load_yaml;
use colored::Colorize;
use yaml_rust::YamlEmitter;

use crate::{
    user_config::rules::{
        actions::ConflictOption,
        rule::Rule,
    },
    PROJECT_NAME,
};

pub fn resolve_name_conflict(filename: &str) -> Result<ConflictOption, Error> {
    print!(
        "A file named {} already exists in the destination.\n [(o)verwrite / (r)ename / (s)kip]: ",
        filename.underline().bold()
    );
    io::stdout().flush().unwrap();

    let mut buf = [0; 1];
    io::stdin().read_exact(&mut buf).unwrap();
    let buf = buf[0];

    if buf == 111 {
        Ok(ConflictOption::Overwrite)
    } else if buf == 114 {
        Ok(ConflictOption::Rename)
    } else if buf == 115 {
        Ok(ConflictOption::Skip)
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "ERROR: invalid option"))
    }
}

/// returns a hashmap where the keys are paths and the values are tuples of rules
/// and indices that indicate the index of the key's corresponding folder in the rule's folders' list
pub fn path2rules(rules: &[Rule]) -> HashMap<&PathBuf, Vec<(&Rule, usize)>> {
    let mut map = HashMap::new();
    for rule in rules.iter() {
        for (i, folder) in rule.folders.iter().enumerate() {
            if !map.contains_key(&folder.path) {
                map.insert(&folder.path, Vec::new());
            }
            map.get_mut(&folder.path).unwrap().push((rule, i));
        }
    }
    map
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
            let config = load_yaml!("../../examples/config.yml");
            let mut output = String::new();
            let mut emitter = YamlEmitter::new(&mut output);
            emitter.dump(config).expect("ERROR: could not create starter config");
            std::fs::write(path, output)?;
        }
        None => panic!("home directory's parent folder should be defined"),
    }
    Ok(())
}

pub(crate) fn prompt_editor_env_var() -> String {
    let platform = env::consts::OS;
    match platform {
        "linux" | "macos" => {
            format!("{} could not find an $EDITOR environment variable or it's not properly set.\nIn your .bashrc (or .zshrc), set 'export EDITOR=$(which <your-favorite-editor-name>) or \
            run {} as 'EDITOR=$(which <your-favorite-editor-name>) {} config'", PROJECT_NAME, PROJECT_NAME, PROJECT_NAME)
        },
        "windows" => format!("{} could not find an EDITOR environment variable or it's not properly set", PROJECT_NAME),
        _ => format!("error: {} not supported", platform)
    }
}
