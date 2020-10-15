use crate::{
    file::File,
    user_config::rules::rule::Rule,
    utils::path2rules,
};
use std::{
    fs,
    io::Error,
};

pub fn run(rules: &[Rule], watching: bool) -> Result<(), Error> {
    let mut path2rules = path2rules(&rules);
    for (path, rules) in path2rules.iter_mut() {
        let files = fs::read_dir(&path)?;
        'files: for file in files {
            let mut file = File::from(file.unwrap().path().as_path());
            if file.path.is_file() {
                'rules: for (rule, index) in rules.iter_mut() {
                    let folder = rule.folders.get(*index).unwrap();
                    let options = &folder.options;
                    if file.is_hidden && !options.hidden_files {
                        continue 'rules;
                    }
                    let filters = &rule.filters;
                    if file.matches_filters(filters) {
                        rule.actions.run(&mut file, watching)?;
                        continue 'files;
                    }
                }
            }
        }
    }
    Ok(())
}
