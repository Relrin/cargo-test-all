use crate::command::Crate;
use crate::error::Result;

pub trait TestRunner {
    fn new(dependency: &Crate) -> Self
    where
        Self: Sized;
    fn setup(&self) -> Result<()>;
    fn run_tests(&self) -> Result<()>;
    fn teardown(&self) -> Result<()>;
}
