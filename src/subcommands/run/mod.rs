use crate::subcommands::config::Rules;
use std::fs;
use std::io::Error;

pub fn run(rules: Rules) -> Result<(), Error> {
    for rule in rules.iter() {
        for folder in rule.folders.iter() {
            let files = fs::read_dir(folder.path.as_ref().unwrap())?;
            for file in files {
                let file = file.unwrap().path();
                let filename = file.file_name().unwrap().to_str().unwrap();
                let is_hidden = filename.starts_with('.');
                let allow_hidden_files = rule.options.as_ref().unwrap().hidden_files.unwrap();
                if !(!file.is_file() || !allow_hidden_files && is_hidden) {
                    println!("{}", file.display());
                }
            }
        }
    }
    Ok(())
}

// pub fn map_folders_to_
