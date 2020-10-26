use clap::App;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Cli {
    pub(crate) config: PathBuf,
    pub(crate) watch: Vec<PathBuf>,
    pub(crate) delay: u8,
    pub(crate) daemon: bool,
}

impl Cli {
    fn validate_config(&self) -> Result<&Self, Error> {
        if self.config.exists() {
            let extension = self.config.extension().ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "invalid config file extension")
            })?;
            if !(extension.eq("yaml") || extension.eq("yaml")) {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "ERROR: invalid config file extension",
                ));
            }
            Ok(self)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "ERROR: config file does not exist",
            ))
        }
    }
    fn validate_watch(&self) -> Result<&Self, Error> {
        for folder in self.watch.iter() {
            if !folder.exists() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "directory {:#?} does not exist",
                        folder.to_str().unwrap_or("")
                    ),
                ));
            }
        }
        Ok(self)
    }
    pub fn new() -> Result<Cli, Error> {
        let yaml = load_yaml!("../cli.yaml");
        let app = App::from_yaml(yaml);
        let matches = app.get_matches();
        let config = canonicalize(PathBuf::from(
            matches.value_of("config").unwrap(), // safe unwrap, "config" is required
        ))?;
        let watch: Vec<PathBuf> = matches
            .values_of("watch")
            .unwrap() // safe unwrap, "watch" is required
            .map(|path| {
                canonicalize(PathBuf::from(path))
                    .unwrap_or_else(|_| panic!("{} is not a valid path", path))
            })
            .collect();

        let delay = match matches.value_of("delay") {
            Some(delay) => match delay.parse::<u8>() {
                Ok(delay) => delay,
                Err(_) => return Err(Error::new(ErrorKind::InvalidData, "invalid delay value")),
            },
            None => 3,
        };

        let daemon = matches.is_present("daemon");

        let cli = Cli {
            config,
            watch,
            delay,
            daemon,
        };

        cli.validate_config()?.validate_watch()?;

        Ok(cli)
    }
}
