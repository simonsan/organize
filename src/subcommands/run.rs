use crate::{
    configuration::{
        folder2rules,
        rules::Rule,
    },
    file::File,
};
use std::{
    fs,
    io::Error,
};

pub fn run(rules: Vec<Rule>) -> Result<(), Error> {
    let folder2rules = folder2rules(&rules);
    for (folder, rules) in folder2rules {
        let files = fs::read_dir(&folder.path)?;
        let options = &folder.options;
        'files: for file in files {
            let mut file = File::from(file.unwrap().path().as_path());
            if file.path.is_file() {
                if file.is_hidden && !options.hidden_files {
                    continue 'files;
                }
                for rule in rules.iter() {
                    let filters = &rule.filters;
                    if file.matches_filters(filters) {
                        rule.actions.run(&mut file)?;
                        continue 'files;
                    }
                }
            }
        }
    }
    Ok(())
}
