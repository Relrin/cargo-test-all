use std::path::{Path, PathBuf};
use std::process::Command;

use failure::ResultExt;
use rustc_serialize::json::Json;

use crate::error::{ErrorKind, Result};

pub fn get_cargo_lock_path() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .arg("locate-project")
        .output()
        .context(ErrorKind::InvalidCommand {
            description: String::from("can't execute the `cargo locate-project` command."),
        })?;

    let response =
        String::from_utf8(output.stdout.clone()).with_context(|err| ErrorKind::Utf8 {
            value: output.stdout,
            index: err.clone().utf8_error().valid_up_to(),
        })?;

    let json = Json::from_str(&response).context(ErrorKind::Other {
        description: String::from(
            "Can't parse a response from the `cargo locate-project` command.",
        ),
    })?;
    let cargo_toml_path = json["root"].as_string().unwrap_or("").to_string();
    let cargo_lock_path = Path::new(&cargo_toml_path)
        .parent()
        .expect("An attempt to get a parent for the root directory.")
        .join("Cargo.lock");

    Ok(cargo_lock_path.to_path_buf())
}
