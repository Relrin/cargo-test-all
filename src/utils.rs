use std::process::Command;

use failure::ResultExt;
use rustc_serialize::json::Json;

use crate::error::{ErrorKind, Result};

fn get_config() -> Result<()> {
    let path = get_config_path()?;
    println!("{:?}", path);
    Ok(())
}

fn get_config_path() -> Result<String> {
    let output = Command::new("cargo")
         .arg("locate-project")
         .output()
         .context(ErrorKind::InvalidCommand {
            description: String::from("can't execute the `cargo locate-project` command.")
         })?;

    let response = String::from_utf8(output.stdout.clone())
        .with_context(|err| ErrorKind::Utf8 {
            value: output.stdout,
            index: err.clone().utf8_error().valid_up_to()
        })?;

    let json = Json::from_str(&response)
        .context(ErrorKind::Other {
           description: String::from("Can't parse a response from the `cargo locate-project` command.")
        })?;
    Ok(json["root"].as_string().unwrap_or("").to_string())
}