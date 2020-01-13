use crate::command::Crate;
use crate::error::Result;
use crate::runners::traits::TestRunner;

pub struct CratesIoDependencyTestRunner;

impl TestRunner for CratesIoDependencyTestRunner {
    fn new(dependency: &Crate) -> Self {
        CratesIoDependencyTestRunner {}
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
