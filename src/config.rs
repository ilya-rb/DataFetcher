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

        let config_path = args.get(1).ok_or(Error::missing_config_path())?;

        let config_path = Path::new(&config_path);
        let ext = config_path
            .extension()
            .ok_or(Error::invalid_config_format())?;

        if ext.ne(CONFIG_FILE_EXT) {
            return Err(Error::invalid_config_format());
        }

        Ok(config_path.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::errors::Error;
    use std::any::Any;

    #[test]
    fn parse_config_path_should_return_valid_path() {
        let path = "path/to/config.json".to_string();
        let args = vec!["api_fetcher".to_string(), path.clone()];
        let result = Config::parse_env_config_path(&args);

        assert!(result.is_ok());
        assert!(result.unwrap().to_str().unwrap().eq(&path));
    }

    #[test]
    fn parse_config_path_should_return_invalid_ext_error() {
        let path = "path/to/config.txt".to_string();
        let args = vec!["arg".to_string(), path.clone()];
        let result = Config::parse_env_config_path(&args);

        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .type_id()
            .eq(&Error::invalid_config_format().type_id()));
    }

    #[test]
    fn parse_config_should_return_missing_path_err() {
        let args = vec!["arg".to_string()];
        let result = Config::parse_env_config_path(&args);
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .type_id()
            .eq(&Error::missing_config_path().type_id()));
    }
}
