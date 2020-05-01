use crate::errors::Error;
use crate::types::Result;
use crate::yaml_config::YamlConfig;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;

pub const CONFIG_FILE_EXT: &str = "yml";

pub const ERR_CONFIG_PATH_FILE_MISSING: &str = "\
[ERROR] :: Missing path to configuration file
Usage: ./data_fetcher path/to/config.yml [FLAGS]
Flags: --verbose (enable logging), --force (Re-downloadd all requests)
";

pub const ERR_INVALID_CONFIG_FORMAT: &str = "\
[ERROR] :: The config must be a valid YAML file, ensure your config has a .yml
extension and contains valid confiig object.
";

#[derive(Debug)]
pub struct Config {
    pub dst: String,
    pub requests: Requests,
}

#[derive(Debug)]
pub struct Requests {
    pub headers: HashMap<String, String>,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug)]
pub struct Endpoint {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub method: reqwest::Method,
}

impl Config {
    pub fn from_args(args: &Vec<String>) -> Result<Config> {
        use std::fs::File;
        use std::io::{self, Read};

        let file_path = Config::parse_env_config_path(args)?;

        let mut config_file = File::open(file_path)?;
        let mut buf = String::new();
        config_file.read_to_string(&mut buf)?;

        let yaml_config: Result<YamlConfig> = serde_yaml::from_str(&buf)
            .map_err(|e| Error::IoError(io::Error::new(io::ErrorKind::InvalidData, e)));

        Ok(yaml_config?.as_config())
    }

    fn parse_env_config_path(args: &Vec<String>) -> Result<PathBuf> {
        use std::path::Path;

        use crate::errors::AppErrorType::{InvalidConfigFileFormat, MissingConfig};
        use crate::errors::Error::AppError;

        let config_path = args
            .get(1)
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

impl PartialOrd for Endpoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.url.partial_cmp(&other.url)
    }
}

impl PartialEq for Endpoint {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::errors::AppErrorType;

    #[test]
    fn parse_config_path_should_return_valid_path() {
        let path = "path/to/config.yml".to_string();
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
