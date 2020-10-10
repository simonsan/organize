use crate::{
    configuration::{
        actions::process_actions,
        Rule,
    },
    file::File,
};
use std::{
    fs,
    io::Error,
};

pub fn run(rules: Vec<Rule>) -> Result<(), Error> {
    for rule in rules.iter() {
        let actions = &rule.actions;
        for folder in rule.folders.iter() {
            let allow_hidden_files = folder.options.hidden_files;
            let files = fs::read_dir(&folder.path)?;

            'files: for file in files {
                let path = file.unwrap().path();
                let mut file = File::from(&path)?;
                if file.path.is_file() {
                    if file.is_hidden && !allow_hidden_files {
                        continue 'files;
                    }
                    if file.matches_filters(&rule.filters) {
                        process_actions(actions, &mut file)?;
                    }
                }
            }
        }
    }
    Ok(())
}
