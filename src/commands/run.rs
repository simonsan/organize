use std::{
    fs,
    io::Error,
    path::Path,
};

use dialoguer::{
    theme::ColorfulTheme,
    Select,
};

use crate::{
    commands::SubCommands,
    file::File,
    user_config::{
        rules::actions::ConflictOption,
        UserConfig,
    },
};
use clap::ArgMatches;

pub fn run(args: &ArgMatches) -> Result<(), Error> {
    let config = UserConfig::new(args)?;
    let path2rules = config.to_map();
    let subcommand = SubCommands::from(args);

    for (path, rules) in path2rules.iter() {
        let files = fs::read_dir(&path)?;
        'files: for file in files {
            let mut file = File::from(file.unwrap().path().as_path());
            if file.path.is_file() {
                'rules: for (rule, index) in rules.iter() {
                    let folder = rule.folders.get(*index).unwrap();
                    let options = &folder.options;
                    if file.is_hidden && !options.hidden_files {
                        continue 'rules;
                    }
                    let filters = &rule.filters;
                    if file.matches_filters(filters) {
                        rule.actions.run(&mut file, subcommand == SubCommands::Watch)?;
                        continue 'files;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn resolve_conflict(path: &Path) -> ConflictOption {
    let selections = ["Overwrite", "Rename", "Skip"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "A file named {} already exists in {}.\nSelect an option and press Enter to resolve this issue:",
            path.file_name().unwrap().to_str().unwrap(),
            if path.is_dir() {
                path.display()
            } else {
                path.parent().unwrap().display()
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
