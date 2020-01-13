use std::process::{Command, Output};

use crate::command::Crate;
use crate::error::{ErrorKind, Result};
use failure::ResultExt;
use rustc_serialize::json::JsonEvent::Error;

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
}
