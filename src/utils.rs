use std::process::Command;

use rustc_serialize::json::Json;

fn get_config() {
    let path = get_config_path();
}

fn get_config_path() -> String {
    let output = Command::new("cargo")
         .arg("locate-project")
         .output()
         .expect("failed to run the `cargo locate-project` command.");

    let json = match Json::from_str(&*output) {
        Ok(j) => j,
        Err(_) => panic!("Can't process response from the `cargo locate-project` command.")
    };
    json["root"].as_string().unwrap().to_string()
}