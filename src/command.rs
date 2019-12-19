use std::path::Path;
use std::str::FromStr;

use cargo_lock::lockfile::Lockfile;
use cargo_lock::package::Source;
use failure::ResultExt;
use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{ErrorKind, Result};
use crate::util::get_cargo_lock_path;
use cargo_lock::Package;

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(
        r"(?P<source_type>.+)\+(?P<url>[^\?\#]+)\??((tag=(?P<tag>[^#]+))?(#(?P<commit>.+)))?"
    )
    .unwrap();
}

#[derive(Debug, Clone)]
enum DependencyTypeEnum {
    CratesIo,
    Git(SourceOptions),
    Local,
}

#[derive(Debug, Clone)]
struct SourceOptions {
    tag: Option<String>,
    commit: Option<String>,
}

#[derive(Debug, Clone)]
struct Crate {
    name: String,
    path: String,
    dependency_type: DependencyTypeEnum,
}

impl From<Package> for Crate {
    fn from(package: Package) -> Self {
        let mut source = package
            .source
            .unwrap_or(Source::from_str("").unwrap())
            .to_string();

        let dependency_type = match URL_REGEX.captures(&source.clone()) {
            Some(captures) => match &captures["source_type"] {
                "git" => {
                    source = match captures.name("url") {
                        Some(value) => value.as_str().to_string(),
                        _ => source,
                    };
                    let tag = match captures.name("tag") {
                        Some(value) => Some(value.as_str().to_string()),
                        _ => None,
                    };
                    let commit = match captures.name("commit") {
                        Some(value) => Some(value.as_str().to_string()),
                        _ => None,
                    };
                    DependencyTypeEnum::Git(SourceOptions { tag, commit })
                }
                _ => DependencyTypeEnum::CratesIo,
            },
            None => DependencyTypeEnum::Local,
        };

        Crate {
            name: package.name.as_str().to_string(),
            path: format!("{}", source),
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
        let cargo_lock = Lockfile::load(path).with_context(|err| {
            let message = format!("Can't parse Cargo.lock file. Reason: {:?}", err);
            ErrorKind::Io { reason: message }
        })?;

        // TODO: Filter by defined dependencies in Cargo.toml
        // TODO: Get sources from Cargo.toml dependencies section when used local dependency
        let used_crates = cargo_lock
            .packages
            .into_iter()
            .map(|package| Crate::from(package.to_owned()))
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
    let cargo_lock_path = get_cargo_lock_path()?;
    let crates = CrateList::load(cargo_lock_path.as_path())?.with_filter_crates(&options.test_only);
    Ok(())
}
