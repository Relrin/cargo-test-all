use crate::command::Crate;
use crate::error::{Error, ErrorKind, Result};

pub fn run_crate_tests(used_crate: Crate) -> Result<Crate> {
    // Clone project

    // Cargo test

    // Cleanup

    Err(Error::from(ErrorKind::TestsFailure {
        crate_name: used_crate.get_name(),
        output: vec![String::from("YOLO")],
    }))

    //Ok(used_crate)
}
