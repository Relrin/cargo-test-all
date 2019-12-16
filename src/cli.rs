use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "test-all",
    version = "0.1.0",
    about = "Cargo extension for running tests of the used dependencies",
)]
pub struct CliOptions {
    #[structopt(
        short = "t",
        long = "threads",
        help = "An amount of concurrent threads for testing crates.",
        default_value = "1"
    )]
    pub threads: u8,
    #[structopt(
        long = "only",
        help = "List of certain crates for testing, separated by comma.",
    )]
    pub only: Option<String>
}
