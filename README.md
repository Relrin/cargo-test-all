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