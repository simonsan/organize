use clap::App;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Cli {
    pub(crate) config: PathBuf,
    pub(crate) watch: Vec<PathBuf>,
    pub(crate) delay: u8,
}

impl Cli {
    pub fn new() -> Result<Cli, Error> {
        let yaml = load_yaml!("../cli.yaml");
        let app = App::from_yaml(yaml);
        let matches = app.get_matches();
        let config = canonicalize(PathBuf::from(matches.value_of("config").unwrap()))?;
        if config.exists() {
            if config.extension().is_some()
                && (config.extension().unwrap().eq("yaml") || config.extension().unwrap().eq("yml"))
            {
                let watch: Vec<PathBuf> = matches.values_of("watch").unwrap().map(|path| canonicalize(PathBuf::from(path)).unwrap()).collect();
                let delay = match matches.value_of("delay") {
                    Some(time) => {
                        time.parse::<u8>().unwrap()
                    },
                    None => {
                        3
                    }
                };
                Ok(Cli { config, watch, delay })
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
