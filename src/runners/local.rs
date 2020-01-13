use std::env::set_current_dir;

use crate::command::Crate;
use crate::error::Result;
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
        self.run_cargo_test_command()
    }

    fn teardown(&self) -> Result<()> {
        Ok(())
    }
}
