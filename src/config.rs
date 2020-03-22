use crate::errors::Error;
use crate::types::Result;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

pub const CONFIG_FILE_EXT: &str = "json";

pub const ERR_CONFIG_PATH_FILE_MISSING: &str = "\
[ERROR] :: Missing path to configuration file
Usage: ./data_fetcher path/to/config.json [FLAGS]
Flags: --verbose (enable logging), --force (Re-downloadd all requests)
";

pub const ERR_INVALID_CONFIG_FORMAT: &str = "\
[ERROR] :: The config must be a valid JSON file, ensure your config has a .json
extension and contains valid confiig object.
";

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
  pub fn from_args(args: &Vec<String>) -> Result<Config> {
    use std::fs::File;
    use std::io::Read;

    let file_path = Config::parse_env_config_path(args)?;
    
    let mut config_file = File::open(file_path)?;
    let mut buf = String::new();
    config_file.read_to_string(&mut buf)?;

    serde_json::from_str(&buf).map_err(|e| Error::IoError(e.into()))
  }

  fn parse_env_config_path(args: &Vec<String>) -> Result<PathBuf> {
    use std::path::Path;

    use crate::errors::Error::AppError;
    use crate::errors::AppErrorType::MissingConfig;
    use crate::errors::AppErrorType::InvalidConfigFileFormat;

    let config_path = args.get(1)
      .ok_or(AppError(MissingConfig, ERR_CONFIG_PATH_FILE_MISSING))?;

    let config_path = Path::new(&config_path);
    let ext = config_path
      .extension()
      .ok_or(AppError(InvalidConfigFileFormat, ERR_INVALID_CONFIG_FORMAT))?;

    if ext.ne(CONFIG_FILE_EXT) {
      return Err(AppError(InvalidConfigFileFormat, ERR_INVALID_CONFIG_FORMAT));
    }

    Ok(config_path.to_owned())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::errors::AppErrorType;

  #[test]
  fn parse_config_path_should_return_valid_path() {
    let path = "path/to/config.json".to_string();
    let args = vec!["api_fetcher".to_string(), path.clone()];
    let result = Config::parse_env_config_path(&args);

    assert!(result.unwrap().to_str().unwrap().eq(&path));
  }

  #[test]
  fn parse_config_path_should_return_invalid_ext_error() {
    let path = "path/to/config.txt".to_string();
    let args = vec!["arg".to_string(), path.clone()];
    let result = Config::parse_env_config_path(&args);

    assert_eq!(result.err().unwrap(), AppErrorType::InvalidConfigFileFormat);
  }

  #[test]
  fn parse_config_should_return_missing_path_err() {
    let args = vec!["arg".to_string()];
    let result = Config::parse_env_config_path(&args);
    
    assert_eq!(result.err().unwrap(), AppErrorType::MissingConfig);
  }
}
