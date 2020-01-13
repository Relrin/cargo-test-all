use std::env::set_current_dir;

use crate::command::Crate;
use crate::error::{Error, ErrorKind, Result};
use crate::runners::traits::TestRunner;

pub struct LocalDependencyTestRunner {
    crate_name: String,
    sources_directory: String,
}

impl TestRunner for LocalDependencyTestRunner {
    fn new(dependency: &Crate) -> Self {
        LocalDependencyTestRunner {
            crate_name: dependency.get_name(),
            sources_directory: dependency.get_path(),
        }
    }

    fn setup(&self) -> Result<()> {
        set_current_dir(self.sources_directory.clone())?;
        Ok(())
    }

    fn run_tests(&self) -> Result<()> {
        let output = self.run_cargo_command("test")?;

        match output.status.success() {
            true => Ok(()),
            false => Err(Error::from(ErrorKind::TestsFailure {
                crate_name: self.crate_name.to_owned(),
                output: String::from_utf8_lossy(&output.stdout).to_string(),
            })),
        }
    }

    fn teardown(&self) -> Result<()> {
        Ok(())
    }
}
