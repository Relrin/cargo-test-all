use crate::command::Crate;
use crate::error::Result;
use crate::runners::traits::TestRunner;

pub struct LocalDependencyTestRunner;

impl TestRunner for LocalDependencyTestRunner {
    fn new(dependency: &Crate) -> Self {
        LocalDependencyTestRunner {}
    }

    fn setup(&self) -> Result<()> {
        Ok(())
    }

    fn run_tests(&self) -> Result<()> {
        Ok(())
    }

    fn teardown(&self) -> Result<()> {
        Ok(())
    }
}
