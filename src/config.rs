use crate::errors::Error;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

pub const CONFIG_FILE_EXT: &str = "json";

#[derive(Deserialize, Debug)]
pub struct Config {
  pub dst: Option<String>,
  pub requests: Option<Requests>,
}

#[derive(Deserialize, Debug)]
pub struct Requests {
  pub headers: Option<HashMap<String, String>>,
  pub endpoints: Option<Vec<Endpoint>>,
}

#[derive(Deserialize, Debug)]
pub struct Endpoint {
  pub url: String,
  pub headers: Option<HashMap<String, String>>,
}

impl Config {
  pub fn new() -> Result<Config, Error> {
    use std::fs::File;
    use std::io::Read;

    let file_path = Config::parse_env_config_path()?;

    let mut config_file = File::open(file_path)?;
    let mut buf = String::new();

    config_file.read_to_string(&mut buf)?;

    match serde_json::from_str(&buf) {
      Ok(s) => Ok(s),
      Err(e) => Err(Error::IoError(e.into())),
    }
  }

  pub fn validate(&self) -> Result<(), Error> {
    if self.dst.is_none() {
      Err(Error::missing_dst())
    } else if self.requests.is_none() {
      Err(Error::missing_requests())
    } else {
      Ok(())
    }
  }

  fn parse_env_config_path() -> Result<PathBuf, Error> {
    use std::ffi::OsStr;
    use std::env;
    use std::path::Path;

    let config_path: String = env::args()
        .skip(1)
        .collect();

    if config_path.is_empty() {
      return Err(Error::missing_config_path());
    }

    let config_path = Path::new(&config_path);
    let ext = config_path.extension();

    if ext.is_none() || ext.unwrap().ne(CONFIG_FILE_EXT) {
      return Err(Error::invalid_config_format());
    }

    Ok(config_path.to_owned())
  }
}