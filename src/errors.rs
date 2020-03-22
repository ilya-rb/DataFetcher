#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  AppError(AppErrorType, &'static str),
}

#[derive(Debug)]
pub enum AppErrorType {
  /// Config path file is not specified in env arguments
  MissingConfig,
  
  /// Config that passed is not a json file 
  /// or invalid json file
  InvalidConfigFileFormat,
  
  /// Error when trying to parse request url as HTTP url
  /// (example: missing host)
  UrlParseError,
  
  /// Error when configuring reqwest client for the request
  UnableToCreateHttpClient,
  
  /// Error when making http request 
  /// (example: missing internet connection
  RequestFetchError,
  
  /// Error trying to parse request body as text 
  /// (probably some internal reqwest error)
  ResponseParseError,
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
      Error::AppError(_, e) => write!(f, "{}", e)
    }
  }
}
