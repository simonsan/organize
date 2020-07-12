use clap::App;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Cli {
    pub(crate) config: PathBuf,
    pub(crate) watch: bool,
    pub(crate) delay: Option<u8>,
}

impl Cli {
    fn validate_config(self) -> Result<Self, Error> {
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
            return Ok(self);
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            "ERROR: config file does not exist",
        ))
    }

    pub fn new() -> Result<Cli, Error> {
        let yaml = load_yaml!("../cli.yaml");
        let app = App::from_yaml(yaml);
        let matches = app.get_matches();
        let config = canonicalize(PathBuf::from(
            matches.value_of("config").unwrap(), // safe unwrap, "config" is required
        ))?;

        let watch = matches.values_of("watch").is_some(); // safe unwrap, "watch" is required
        let mut delay: Option<u8> = None;
        if watch {
            delay = Some(match matches.value_of("delay") {
                Some(delay) => match delay.parse::<u8>() {
                    Ok(delay) => delay,
                    Err(_) => {
                        return Err(Error::new(ErrorKind::InvalidData, "invalid delay value"))
                    }
                },
                None => 3,
            })
        }

        let cli = Cli {
            config,
            watch,
            delay,
        };

        Ok(cli.validate_config()?)
    }
}
