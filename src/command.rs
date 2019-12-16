use std::sync::{Arc, Mutex};

use crate::error::Result;
use crate::util::get_project_path;

enum DependencyTypeEnum {
    CratesIo,
    Git,
    Local,
}

struct Crate {
    name: String,
    version: String,
    path: String,
    dependency_type: DependencyTypeEnum,
}

struct CrateList {
    all: Arc<Mutex<Vec<Crate>>>,
    passed: Arc<Mutex<Vec<Crate>>>,
    failed: Arc<Mutex<Vec<Crate>>>,
}

pub struct TestOptions {
    pub threads: u8,
    pub test_only: Vec<String>,
}

pub fn test_crates(options: &TestOptions) -> Result<()> {
    let path = get_project_path()?;
    println!("{:?}", path);

    let tested_crates = CrateList {
        all: Arc::new(Mutex::new(Vec::new())),
        passed: Arc::new(Mutex::new(Vec::new())),
        failed: Arc::new(Mutex::new(Vec::new())),
    };

    Ok(())
}
