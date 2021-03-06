use failure;
#[macro_use]
extern crate failure_derive;
use serde_json;

pub mod errors;

use serde_json::Value;
use std::{fs, path::Path, process::Command, result};

pub type Result<T> = result::Result<T, errors::Error>;

pub fn get_track_root(track_dir: &Path) -> Result<String> {
    let rev_parse_output = Command::new("git")
        .current_dir(track_dir)
        .args(&["rev-parse", "--show-toplevel"])
        .output()?;

    Ok(String::from_utf8(rev_parse_output.stdout)?
        .trim()
        .to_string())
}

pub fn get_config_value(track_dir: &Path) -> Result<Value> {
    let track_root = get_track_root(track_dir)?;

    let config_path = Path::new(&track_root).join("config.json");

    let config_content = fs::read_to_string(config_path)?;

    Ok(serde_json::from_str(&config_content)?)
}
