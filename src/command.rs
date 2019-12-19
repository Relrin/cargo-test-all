use std::path::Path;

use cargo::core::{Dependency, GitReference};

use crate::error::Result;
use crate::util::{get_project_location, load_cargo_toml};

#[derive(Debug, Clone)]
enum DependencyTypeEnum {
    CratesIo,
    Git(SourceOptions),
    Local,
}

#[derive(Debug, Clone)]
struct SourceOptions {
    branch: Option<String>,
    tag: Option<String>,
    commit: Option<String>,
}

#[derive(Debug, Clone)]
struct Crate {
    name: String,
    path: String,
    dependency_type: DependencyTypeEnum,
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
struct CrateList {
    all: Vec<Crate>,
    passed: Vec<Crate>,
    failed: Vec<Crate>,
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
            all: used_crates,
            passed: Vec::new(),
            failed: Vec::new(),
        })
    }

    pub fn with_filter_crates(mut self, test_only: &Vec<String>) -> Self {
        match test_only.is_empty() {
            true => (),
            false => {
                self.all = self
                    .all
                    .into_iter()
                    .filter(|obj| test_only.contains(&obj.name))
                    .collect();
            }
        };

        self
    }
}

#[derive(Debug, Clone)]
pub struct TestOptions {
    pub threads: u8,
    pub test_only: Vec<String>,
}

pub fn test_crates(options: &TestOptions) -> Result<()> {
    let project_location = get_project_location()?;
    let crates =
        CrateList::load(project_location.as_path())?.with_filter_crates(&options.test_only);
    Ok(())
}
