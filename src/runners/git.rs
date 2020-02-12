use std::env::{current_dir, set_current_dir};
use std::path::PathBuf;
use std::process::Command;

use failure::ResultExt;

use crate::command::{Crate, DependencyTypeEnum, SourceOptions};
use crate::error::{Error, ErrorKind, Result};
use crate::runners::traits::TestRunner;

pub struct GitDependencyTestRunner {
    crate_name: String,
    url: String,
    source_options: SourceOptions,
    parent_directory: String,
    target_directory: String,
}

impl TestRunner for GitDependencyTestRunner {
    fn new(dependency: &Crate) -> Self {
        let source_options = match dependency.get_dependency_type() {
            DependencyTypeEnum::Git(options) => options,
            _ => SourceOptions::default(),
        };

        let current_directory = current_dir().unwrap();
        let parent_directory = current_directory.to_str().unwrap().to_string();
        let target_directory = current_directory
            .join(dependency.get_name().clone())
            .to_str()
            .unwrap()
            .to_string();

        GitDependencyTestRunner {
            crate_name: dependency.get_name(),
            url: dependency.get_path(),
            source_options,
            parent_directory,
            target_directory,
        }
    }

    fn setup(&self) -> Result<()> {
        let target_directory = self.target_directory.clone();
        let deps_directory = PathBuf::from(target_directory);
        set_current_dir(deps_directory.parent().unwrap())?;

        let mut command_args = Vec::new();

        command_args.push("--prefix".to_string());
        command_args.push(self.target_directory.clone());

        if let Some(branch) = self.source_options.get_branch() {
            command_args.push("--branch".to_string());
            command_args.push(branch.clone());
        }

        if let Some(tag) = self.source_options.get_tag() {
            command_args.push("--tag".to_string());
            command_args.push(tag.clone());
        }

        if let Some(commit) = self.source_options.get_commit() {
            command_args.push("--rev".to_string());
            command_args.push(commit.clone());
        }

        command_args.push("--git".to_string());
        command_args.push(self.url.clone());

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
