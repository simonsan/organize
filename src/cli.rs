use clap::App;
use std::fs::canonicalize;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
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
        let config = canonicalize(PathBuf::from(
            matches.value_of("config").unwrap(), // safe unwrap, "config" is required
        ))?;

        if config.exists() {
            let extension = config.extension().ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "invalid config file extension")
            })?;
            if extension.eq("yaml") || extension.eq("yaml") {
                let watch: Vec<PathBuf> = matches
                    .values_of("watch")
                    .unwrap() // safe unwrap, "watch" is required
                    .map(|path| {
                        canonicalize(PathBuf::from(path))
                            .unwrap_or_else(|_| panic!("{} is not a valid path", path))
                    })
                    .collect();

                let delay = matches
                    .value_of("delay")
                    .unwrap_or("3")
                    .parse::<u8>()
                    .unwrap(); // safe unwrap, "3" cannot fail to be parsed

                Ok(Cli {
                    config,
                    watch,
                    delay,
                })
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "ERROR: invalid config file extension",
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
