use std::path::{Path, PathBuf};
use std::process::Command;

use cargo::core::{EitherManifest, Manifest, SourceId};
use cargo::util::toml::read_manifest;
use cargo::Config;
use failure::ResultExt;
use rustc_serialize::json::Json;

use crate::error::{Error, ErrorKind, Result};

pub fn load_cargo_toml(path: &Path) -> Result<Manifest> {
    let config = Config::default().expect("Unable to get config.");
    let source_id = SourceId::for_path(path).with_context(|err| {
        let message = format!(
            "Can't generate SourceId for Cargo.toml file. Reason: {:?}",
            err
        );
        ErrorKind::Io { reason: message }
    })?;

    let (manifest, _path) = read_manifest(path, source_id, &config).with_context(|err| {
        let message = format!("Can't read Cargo.toml file. Reason: {:?}", err);
        ErrorKind::Io { reason: message }
    })?;

    let cargo_toml = match manifest {
        EitherManifest::Real(manifest) => manifest,
        _ => {
            let description = String::from("Received a virtual Cargo.toml data.");
            return Err(Error::from(ErrorKind::Other { description }));
        }
    };

    Ok(cargo_toml)
}

pub fn get_project_location() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .arg("locate-project")
        .output()
        .context(ErrorKind::InvalidCommand {
            description: String::from("Can't execute the `cargo locate-project` command."),
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
    let project_location = Path::new(&cargo_toml_path)
        .parent()
        .expect("An attempt to get a parent for the root directory.");

    Ok(project_location.to_path_buf())
}
