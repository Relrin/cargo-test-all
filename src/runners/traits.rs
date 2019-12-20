use crate::command::Crate;

pub trait TestRunner {
    fn new(crate: &Crate) -> Self
    where
        Self: Sized;
    fn run_tests(&self);
    fn setup(&self);
    fn teardown(&self);
}
