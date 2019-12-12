use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "test-all",
    version = "1.1.0",
    about = "Cargo extension for running tests of the used dependencies",
)]
pub struct CliOptions {
    #[structopt(
        short = "t",
        long = "threads",
        help = "The listened port",
        default_value = "1"
    )]
    pub threads: u8,
}
