use std::{
    collections::HashMap,
    env,
    io::{
        Error,
        ErrorKind,
    },
    path::{
        Path,
        PathBuf,
    },
};

use clap::load_yaml;
use yaml_rust::YamlEmitter;

use crate::{
    user_config::rules::{
        actions::ConflictOption,
        rule::Rule,
    },
    PROJECT_NAME,
};
use dialoguer::{
    theme::ColorfulTheme,
    Select,
};

pub fn resolve_name_conflict(from: &Path, to: &Path) -> ConflictOption {
    let selections = ["Overwrite", "Rename", "Skip"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "A file named {} already exists in {}.\nSelect an option and press Enter to resolve this issue:",
            from.file_name().unwrap().to_str().unwrap(),
            if to.is_dir() {
                to.display()
            } else {
                to.parent().unwrap().display()
            }
        ))
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => ConflictOption::Overwrite,
        1 => ConflictOption::Rename,
        2 => ConflictOption::Skip,
        _ => panic!("no option selected"),
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

#[cfg(test)]
pub fn project_dir() -> PathBuf {
    // 'cargo test' must be run from the project directory, where Cargo.toml is
    // even if you run it from some other folder inside the project
    // 'cargo test' will move to the project root
    env::current_dir().unwrap()
}

#[cfg(test)]
pub fn tests_dir() -> PathBuf {
    project_dir().join("tests")
}

#[cfg(test)]
pub fn test_file_or_dir(filename: &str) -> PathBuf {
    tests_dir().join("files").join(filename)
}
