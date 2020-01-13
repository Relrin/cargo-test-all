use std::process::{Command, Output};

use failure::ResultExt;

use crate::command::Crate;
use crate::error::{Error, ErrorKind, Result};

pub trait TestRunner {
    fn new(dependency: &Crate) -> Self
    where
        Self: Sized;

    fn setup(&self) -> Result<()>;

    fn run_tests(&self) -> Result<()>;

    fn teardown(&self) -> Result<()>;

    fn run_cargo_command(&self, command: &str) -> Result<Output> {
        let output = Command::new("cargo")
            .arg(command)
            .output()
            .with_context(|err| ErrorKind::Io {
                reason: format!("{}", err),
            })?;

        Ok(output)
    }

    fn run_cargo_test_command(&self) -> Result<()> {
        let output = self.run_cargo_command("test")?;

        match output.status.success() {
            true => Ok(()),
            false => Err(Error::from(ErrorKind::TestsFailure {
                crate_name: self.crate_name.to_owned(),
                output: String::from_utf8_lossy(&output.stdout).to_string(),
            })),
        }
    }
}
