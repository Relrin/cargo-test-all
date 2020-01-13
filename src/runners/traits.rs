use std::process::{Command, Stdio};

use crate::command::Crate;
use crate::error::{ErrorKind, Result};
use failure::ResultExt;

pub trait TestRunner {
    fn new(dependency: &Crate) -> Self
    where
        Self: Sized;

    fn setup(&self) -> Result<()>;

    fn run_tests(&self) -> Result<()>;

    fn teardown(&self) -> Result<()>;

    fn run_cargo_command(&self, command: &str) -> Result<()> {
        let mut process = Command::new("cargo")
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        process.wait().with_context(|err| ErrorKind::Io {
            reason: format!("{}", err),
        })?;
        Ok(())
    }
}
