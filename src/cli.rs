use clap::App;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Cli {
    pub(crate) config: PathBuf,
    pub(crate) watch: PathBuf,
    pub(crate) delay: u64,
}

impl Cli {
    pub fn new() -> Result<Cli, Error> {
        let yaml = load_yaml!("../cli.yaml");
        let matches = App::from_yaml(yaml).get_matches();
        let config = canonicalize(PathBuf::from(matches.value_of("config").unwrap()))?;
        if config.exists() {
            if config.extension().is_some()
                && (config.extension().unwrap().eq("yaml") || config.extension().unwrap().eq("yml"))
            {
                let watch = canonicalize(PathBuf::from(matches.value_of("watch").unwrap()))?;
                let delay = matches.value_of("delay").unwrap_or("5000");
                Ok(Cli { config, watch, delay: delay.parse().unwrap() })
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "ERROR: invalid file extension for {:#?}",
                        &config.file_name().unwrap()
                    ),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "ERROR: config file does not exist",
            ))
        }
    }
}
