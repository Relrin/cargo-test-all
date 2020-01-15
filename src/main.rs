mod cli;
mod command;
mod error;
pub mod runners;
mod util;
mod worker;

use structopt::StructOpt;

use crate::cli::CliOptions;
use crate::command::{test_crates, TestOptions};

fn main() {
    let args = CliOptions::from_args();
    let test_only = args
        .only
        .unwrap_or(String::from(""))
        .split(",")
        .map(|s| s.to_string().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let options = TestOptions {
        threads: args.threads,
        test_only,
    };
    match test_crates(&options) {
        Err(err) => println!("{}", err),
        _ => (),
    }
}
