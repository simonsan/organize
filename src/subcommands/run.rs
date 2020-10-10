use crate::{
    configuration::actions::process_actions,
    file::File,
    subcommands::config::Rules,
};
use std::{
    fs,
    io::Error,
};

pub fn run(rules: Rules) -> Result<(), Error> {
    for rule in rules.iter() {
        let filters = rule.filters.as_ref().unwrap();
        let actions = &rule.actions;
        for folder in rule.folders.iter() {
            let options = rule.options.as_ref().unwrap() + folder.options.as_ref().unwrap();
            let allow_hidden_files = options.hidden_files.unwrap();
            let files = fs::read_dir(&folder.path)?;

            'files: for file in files {
                let path = file.unwrap().path();
                let mut file = File::from(&path)?;
                if file.path.is_file() {
                    if file.is_hidden && !allow_hidden_files {
                        continue 'files;
                    }
                    if file.matches_filters(filters) {
                        process_actions(actions, &mut file)?;
                    }
                }
            }
        }
    }
    Ok(())
}
