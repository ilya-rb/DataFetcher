use crate::types::Result;
use crate::errors::Error;

pub struct FileToSave {
  pub file_path: String,
  pub file_name: String
}

pub fn create_dst_file(root_folder_path: &str, url: &str) -> Result<FileToSave> {
  use crate::errors::AppErrorType::UrlParseError;
  use crate::errors::Error::AppError;
  use crate::config::CONFIG_FILE_EXT;
  use reqwest::Url;

  let url = Url::parse(&url).map_err(|_| AppError(UrlParseError, "Unable to parse http url"))?;
  let url_path: Vec<&str> = url.path_segments().unwrap().collect();

  // Using last part of the url as a file name
  let (file_name, url_path) = url_path.split_last().unwrap();

  // Trim ending slash if needed
  let root_folder_path = if root_folder_path.ends_with("/") {
    &root_folder_path[0..root_folder_path.len() - 1] 
  } else {
    root_folder_path 
  };

  let file_path = format!("{}/{}", root_folder_path, url_path.join("/"));
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

#[cfg(test)]
mod test {
  use super::*;
  use crate::errors::AppErrorType;

  #[test]
  fn create_dst_file_should_return_correct_file() {
    let root_folder_path = "dst/root/";
    let url = "https://api.com/endpoint/path";
    let result = create_dst_file(&root_folder_path, &url).unwrap();

    assert_eq!(result.file_path, "dst/root/endpoint");
    assert_eq!(result.file_name, "dst/root/endpoint/path.json");
  }

  #[test]
  fn create_dst_file_should_fail_on_invalid_url() {
    let root_folder_path = "root/";
    let url = "http:/some_invalid_url/";
    let result = create_dst_file(&root_folder_path, &url);

    assert_eq!(result.err().unwrap(), AppErrorType::UrlParseError);
  }
}