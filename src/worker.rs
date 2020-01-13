use crate::command::Crate;
use crate::error::{Error, ErrorKind, Result};
use crate::runners::get_test_runner;

pub fn run_crate_tests(used_crate: Crate) -> Result<Crate> {
    let test_runner = get_test_runner(&used_crate);

    test_runner.setup()?;
    test_runner.run_tests()?;
    test_runner.teardown()?;

    Ok(used_crate)
}
