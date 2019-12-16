use cargo::util::CargoResult;

pub struct TestOptions {
    pub threads: u8,
    pub test_only: Vec<String>,
}

pub fn test_crates(options: &TestOptions) -> CargoResult<()> {
    Ok(())
}
