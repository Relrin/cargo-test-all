mod cli;
mod command;
mod error;
mod util;

use structopt::StructOpt;

use crate::cli::CliOptions;
use crate::command::{test_crates, TestOptions};

fn main() {
    let command = CliOptions::from_args();
    let test_only = command
        .only
        .unwrap_or(String::from(""))
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let options = TestOptions {
        threads: command.threads,
        test_only,
    };
    match test_crates(&options) {
        Err(err) => println!("{}", err),
        _ => (),
    }
}
