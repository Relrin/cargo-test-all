# cargo-test-all
Cargo extension for running tests of the used dependencies

# Features
- Running tests for each dependency, specified in Cargo.toml
- Can be specified only the certain crates for testing
- Have an opportunity to run multiple threads/workers for building and testing crates

# Installation
For installation this executable use the following command in your terminal or shell:
```
cargo install cargo-test-all
```

# Usage
Use the `cargo test-all` call at the toplevel of any Cargo project.

```
Cargo extension for running tests of the used dependencies

USAGE:
    cargo-test-all [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --only <only>          List of certain crates for testing, separated by comma.
    -t, --threads <threads>    An amount of concurrent threads for testing crates. [default: 1]
```

# How it works
Because the Cargo currently does not provide (as far as I aware) any way to install the dependency with its own tests, it works in the following way:
1) From the given output determines which crates needs to test.
2) Creates the `target/testing/deps` directory that will be used for storing crates downloaded from Crates.io or with Git.
3) For each crate:
    1) Download crate from the default storage or via Git (if it isn't local).
    2) Move to the folder with code
    3) Build the sources and run tests as the task, executed by the worker
    4) Results of the finished task stored in the main thread and print them out when everything is done.
