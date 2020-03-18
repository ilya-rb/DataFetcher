const ERR_MISSING_CONFIG_PATH: &str = "\
  Config file path is missing, \
  example of usage ./api_fetcher path/to/config.json\
  ";

const ERR_INVALID_CONFIG_FORMAT: &str = "\
  The config must be a valid .json file\
  ";

#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  InvalidConfigFormatError(&'static str),
  MissingConfigPathError(&'static str),
}

impl Error {
  pub fn missing_config_path() -> Error {
    Error::MissingConfigPathError(ERR_MISSING_CONFIG_PATH)
  }

  pub fn invalid_config_format() -> Error {
    Error::InvalidConfigFormatError(ERR_INVALID_CONFIG_FORMAT)
  }
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Error::IoError(e)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::IoError(e) => e.fmt(f),
      Error::MissingConfigPathError(e) => write!(f, "{}", e),
      Error::InvalidConfigFormatError(e) => write!(f, "{}", e),
    }
  }
}