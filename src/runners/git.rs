use crate::command::Crate;
use crate::error::Result;
use crate::runners::traits::TestRunner;

pub struct GitDependencyTestRunner;

impl TestRunner for GitDependencyTestRunner {
    fn new(dependency: &Crate) -> Self {
        GitDependencyTestRunner {}
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
