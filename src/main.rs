use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "d-organizer")]
struct Cli {
    #[structopt(long = "watch", short = "w")]
    watch: String
}

impl Cli {
    // fn new() -> Result<>
}

fn main() {
    let cli = Cli::from_args();
    println!("{}", &cli.watch);
    println!("Hello, world!");
}

