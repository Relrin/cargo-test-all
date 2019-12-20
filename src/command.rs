use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::mpsc::channel;

use cargo::core::{Dependency, GitReference};
use workerpool::thunk::{Thunk, ThunkWorker};
use workerpool::Pool;

use crate::error::{ErrorKind, Result};
use crate::util::{get_project_location, load_cargo_toml};
use crate::worker::run_crate_tests;

#[derive(Debug, Clone)]
pub enum DependencyTypeEnum {
    CratesIo,
    Git(SourceOptions),
    Local,
}

#[derive(Debug, Clone)]
pub struct SourceOptions {
    branch: Option<String>,
    tag: Option<String>,
    commit: Option<String>,
}

impl SourceOptions {
    pub fn get_branch(&self) -> Option<String> {
        self.branch.clone()
    }

    pub fn get_tag(&self) -> Option<String> {
        self.tag.clone()
    }

    pub fn get_commit(&self) -> Option<String> {
        self.commit.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Crate {
    name: String,
    path: String,
    dependency_type: DependencyTypeEnum,
}

impl Crate {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_dependency_type(&self) -> DependencyTypeEnum {
        self.dependency_type.clone()
    }
}

impl From<Dependency> for Crate {
    fn from(dependency: Dependency) -> Self {
        let name = dependency.package_name().to_string();
        let source_id = dependency.source_id();
        let mut path = source_id.url().to_string();

        let is_git = source_id.is_git();
        let is_registry = source_id.is_registry();
        let is_local = source_id.is_path();
        let dependency_type = match (is_registry, is_git, is_local) {
            (true, _, _) => DependencyTypeEnum::CratesIo,
            (_, true, _) => {
                let mut branch = None;
                let mut tag = None;
                let mut commit = None;
                match source_id.git_reference() {
                    Some(GitReference::Branch(value)) => branch = Some(value.to_owned()),
                    Some(GitReference::Tag(value)) => tag = Some(value.to_owned()),
                    Some(GitReference::Rev(value)) => commit = Some(value.to_owned()),
                    _ => (),
                };
                DependencyTypeEnum::Git(SourceOptions {
                    branch,
                    tag,
                    commit,
                })
            }
            (_, _, true) => {
                path = path.trim_start_matches("file://").to_string();
                DependencyTypeEnum::Local
            }
            (_, _, _) => DependencyTypeEnum::CratesIo,
        };

        Crate {
            name,
            path,
            dependency_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrateList {
    all: Box<Vec<Crate>>,
    failed: Box<Vec<ErrorKind>>,
}

impl CrateList {
    pub fn load(path: &Path) -> Result<Self> {
        let cargo_toml_path = path.join("Cargo.toml");
        let cargo_toml = load_cargo_toml(&cargo_toml_path)?;

        let used_crates = cargo_toml
            .dependencies()
            .into_iter()
            .map(|dependency| Crate::from(dependency.to_owned()))
            .collect::<Vec<Crate>>();

        Ok(CrateList {
            all: Box::new(used_crates),
            failed: Box::new(Vec::new()),
        })
    }

    pub fn with_filter_crates(mut self, test_only: &Vec<String>) -> Self {
        match test_only.is_empty() {
            true => (),
            false => {
                self.all = Box::new(
                    self.all
                        .into_iter()
                        .filter(|obj| test_only.contains(&obj.name))
                        .collect(),
                );
            }
        };

        self
    }

    pub fn get_tested_crates_list(&self) -> &Box<Vec<Crate>> {
        &self.all
    }

    pub fn get_failed_crates(&self) -> &Box<Vec<ErrorKind>> {
        &self.failed
    }

    pub fn append_error(&mut self, error: &ErrorKind) {
        self.failed.push(error.clone());
    }

    pub fn has_failed_tests(&self) -> bool {
        !self.failed.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct TestOptions {
    pub threads: usize,
    pub test_only: Vec<String>,
}

pub fn test_crates(options: &TestOptions) -> Result<()> {
    let project_location = get_project_location()?;
    let mut crate_list =
        CrateList::load(project_location.as_path())?.with_filter_crates(&options.test_only);

    let tested_crates = crate_list.get_tested_crates_list();
    let total_crates = tested_crates.len();
    let pool = Pool::<ThunkWorker<Result<Crate>>>::new(options.threads);
    let (tx, rx) = channel();
    for used_crate in tested_crates.clone().into_iter() {
        pool.execute_to(tx.clone(), Thunk::of(move || run_crate_tests(used_crate)));
    }

    rx.iter()
        .take(tested_crates.len())
        .filter(|response| response.is_err())
        .for_each(|response| {
            let error = response.unwrap_err();
            let error_kind = error.kind();
            crate_list.append_error(error_kind);
        });

    let current_directory = current_dir();

    match crate_list.has_failed_tests() {
        true => {
            let failed_crates = crate_list.get_failed_crates();
            println!("Failed {} of {} crates.", failed_crates.len(), total_crates);
            for error in failed_crates.iter() {
                println!("{}", error)
            }
        }
        false => println!("Well done! All crates work correctly."),
    }

    Ok(())
}
