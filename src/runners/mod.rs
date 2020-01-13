mod cratesio;
mod git;
mod local;
mod traits;

pub use crate::runners::cratesio::CratesIoDependencyTestRunner;
pub use crate::runners::git::GitDependencyTestRunner;
pub use crate::runners::local::LocalDependencyTestRunner;
pub use crate::runners::traits::TestRunner;

use crate::command::{Crate, DependencyTypeEnum};

pub fn get_test_runner(dependency: &Crate) -> Box<dyn TestRunner> {
    match dependency.get_dependency_type() {
        DependencyTypeEnum::CratesIo => Box::new(CratesIoDependencyTestRunner::new(dependency)),
        DependencyTypeEnum::Git(_) => Box::new(GitDependencyTestRunner::new(dependency)),
        DependencyTypeEnum::Local => Box::new(LocalDependencyTestRunner::new(dependency)),
    }
}
