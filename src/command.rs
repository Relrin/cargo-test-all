use std::path::Path;

use cargo_lock::lockfile::Lockfile;
use cargo_lock::package::Source;
use failure::ResultExt;

use crate::error::{ErrorKind, Result};
use crate::util::get_cargo_lock_path;

enum DependencyTypeEnum {
    CratesIo,
    Git(SourceOptions),
    GitLab(SourceOptions),
    BitBucket(SourceOptions),
    Local,
}

struct SourceOptions {
    branch: Option<String>,
    tag: Option<String>,
    commit: Option<String>,
}

struct Crate {
    name: String,
    path: String,
    dependency_type: DependencyTypeEnum,
}

struct CrateList {
    all: Vec<Crate>,
    passed: Vec<Crate>,
    failed: Vec<Crate>,
}

impl CrateList {
    pub fn load(path: &Path) -> Result<Self> {
        let cargo_lock = Lockfile::load(path).with_context(|err| {
            let message = format!("Can't parse Cargo.lock file. Reason: {:?}", err);
            ErrorKind::Io { reason: message }
        })?;

        // TODO: Implement parsing source type
        // TODO: Implement From<Package> for Crate ...
        let used_crates = cargo_lock
            .packages
            .iter()
            .map(|package| Crate {
                name: package.name.as_str().to_string(),
                path: package.source.unwrap_or(Source(String::from(""))).unwrap(),
                dependency_type: DependencyTypeEnum::CratesIo,
            })
            .collect();

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

pub struct TestOptions {
    pub threads: u8,
    pub test_only: Vec<String>,
}

pub fn test_crates(options: &TestOptions) -> Result<()> {
    let cargo_lock_path = get_cargo_lock_path()?;
    let crates = CrateList::load(cargo_lock_path.as_path())?.with_filter_crates(&options.test_only);
    Ok(())
}
