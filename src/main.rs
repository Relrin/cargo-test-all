mod cli;
mod error;
mod util;

use structopt::StructOpt;

use crate::cli::CliOptions;

fn main() {
    let command = CliOptions::from_args();

}
