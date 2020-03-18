use crate::errors::Error;
use crate::types::Result;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

pub const CONFIG_FILE_EXT: &str = "json";

#[derive(Deserialize, Debug)]
pub struct Config {
  pub dst: String,
  pub requests: Requests,
}

#[derive(Deserialize, Debug)]
pub struct Requests {
  pub headers: Option<HashMap<String, String>>,
  pub endpoints: Vec<Endpoint>,
}

#[derive(Deserialize, Debug)]
pub struct Endpoint {
  pub url: String,
  pub headers: Option<HashMap<String, String>>,
}

impl Config {
  pub fn from_env_args() -> Result<Config> {
    use std::fs::File;
    use std::io::Read;

    let file_path = Config::parse_env_config_path()?;

    let mut config_file = File::open(file_path)?;
    let mut buf = String::new();
    config_file.read_to_string(&mut buf)?;

    serde_json::from_str(&buf).map_err(|e| Error::IoError(e.into()))
  }

  fn parse_env_config_path() -> Result<PathBuf> {
    use std::env;
    use std::path::Path;

    let config_path = env::args()
        .nth(1)
        .ok_or(Error::missing_config_path())?;

    let config_path = Path::new(&config_path);
    let ext = config_path.extension().ok_or(Error::invalid_config_format())?;

    if ext.ne(CONFIG_FILE_EXT) {
      return Err(Error::invalid_config_format());
    }

    Ok(config_path.to_owned())
  }
}