use std::any::Any;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::File;
use std::env;

const ERR_MISSING_CONFIG_PATH: &str = "Config file path is missing";
const ERR_INVALID_CONFIG_FORMAT: &str = "Config file must be a .json file";
const ERR_INVALID_JSON_FORMAT: &str = "Config file contains invalid JSON";
const CONFIG_FILE_EXT: &str = "json";

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
  parse_config_path()?;
  Ok(())
}

fn parse_config_path() -> Result<PathBuf, &'static str> {
  let config_path: String = env::args()
      // Skip program's name
      .skip(1)
      .collect();

  if config_path.is_empty() {
    return Err(ERR_MISSING_CONFIG_PATH);
  }

  let path = Path::new(&config_path);
  match path.extension() {
    None => Err(ERR_INVALID_CONFIG_FORMAT),
    Some(ext) => {
      if ext.eq(CONFIG_FILE_EXT) {
        Ok(path.to_owned())
      } else {
        Err(ERR_INVALID_CONFIG_FORMAT)
      }
    }
  }
}
