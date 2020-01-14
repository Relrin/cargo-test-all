use std::env::{current_dir, set_current_dir};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;

use failure::ResultExt;

use crate::command::{Crate, DependencyTypeEnum};
use crate::error::{Error, ErrorKind, Result};
use crate::runners::traits::TestRunner;

pub struct CratesIoDependencyTestRunner {
    crate_name: String,
    version: String,
    parent_directory: String,
    target_directory: String,
}

impl TestRunner for CratesIoDependencyTestRunner {
    fn new(dependency: &Crate) -> Self {
        let version = match dependency.get_dependency_type() {
            DependencyTypeEnum::CratesIo(value) => value.to_owned(),
            _ => unreachable!(),
        };

        let current_directory = current_dir().unwrap();
        let parent_directory = current_directory.to_str().unwrap().to_string();
        let target_directory = current_directory
            .join(dependency.get_name().clone())
            .to_str()
            .unwrap()
            .to_string();

        CratesIoDependencyTestRunner {
            crate_name: dependency.get_name(),
            version,
            parent_directory,
            target_directory,
        }
    }

    fn setup(&self) -> Result<()> {
        let target_directory = self.target_directory.clone();
        create_dir_all(target_directory.clone())?;

        let deps_directory = PathBuf::from(target_directory);
        set_current_dir(deps_directory.parent().unwrap())?;

        let mut command_args = Vec::new();

        command_args.push("--vers".to_string());
        command_args.push(self.version.clone());

        command_args.push(self.crate_name.clone());

        let output = Command::new("cargo")
            .arg("clone")
            .args(&command_args)
            .output()
            .with_context(|err| ErrorKind::Io {
                reason: format!("{}", err),
            })?;

        match output.status.success() {
            true => {
                set_current_dir(self.target_directory.clone())?;
                Ok(())
            }
            false => Err(Error::from(ErrorKind::TestsFailure {
                crate_name: self.crate_name.to_owned(),
                output: String::from_utf8_lossy(&output.stderr).to_string(),
            })),
        }
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
        set_current_dir(self.parent_directory.clone()).unwrap();
        Ok(())
    }
}
