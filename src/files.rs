use crate::config::Config;
use crate::types::Result;
use crate::errors::Error;

pub struct FileToSave {
  pub file_path: String,
  pub file_name: String
}

pub fn create_dst_file(config: &Config, url: &str) -> Result<FileToSave> {
  use crate::errors::AppErrorType::UrlParseError;
  use crate::errors::Error::AppError;
  use crate::config::CONFIG_FILE_EXT;
  use reqwest::Url;

  let url = Url::parse(&url).map_err(|_| AppError(UrlParseError, "Unable to parse http url"))?;

  let url_path: Vec<&str> = url.path_segments().unwrap().collect();
  let (file_name, url_path) = url_path.split_last().unwrap();

  let file_path = format!("{}/{}", config.dst, url_path.join("/"));
  let file_name = format!("{}/{}.{}", file_path, file_name, CONFIG_FILE_EXT);

  Ok(FileToSave { file_path, file_name })
}

pub fn write_response_to_file(dst: FileToSave, response: String) -> Result<()> {
  use std::fs;
  use std::fs::File;
  use std::io::Write;

  if let Err(e) = fs::create_dir_all(dst.file_path) {
    // Ignore AlreadyExists error as we probably re-downloading
    // existing response for the request with --force flag.
    if e.kind() != std::io::ErrorKind::AlreadyExists {
      return Err(Error::IoError(e));
    }
  };

  let mut dst_file = File::create(dst.file_name)?;
  dst_file.write_all(&response.as_bytes())?;

  Ok(())
}