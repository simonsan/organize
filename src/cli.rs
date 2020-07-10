use std::fs::canonicalize;
use std::io::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "d-organizer")]
pub struct Cli {
    /// folder to watch
    #[structopt(long, short, parse(from_os_str))]
    pub watch: PathBuf,
    /// configuration filepath
    #[structopt(long, short, parse(from_os_str))]
    pub config: PathBuf,
}

impl Cli {
    pub fn new() -> Result<Cli, Error> {
        let mut app: Cli = Cli::from_args();
        app.watch = canonicalize(&app.watch)?;
        app.config = canonicalize(&app.config)?;
        Ok(app)
    }
}
