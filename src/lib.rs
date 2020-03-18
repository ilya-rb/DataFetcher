mod config;
mod errors;
mod headers;
mod types;

use config::Config;
use config::Endpoint;
use errors::Error;
use types::Result;

pub fn run() -> Result<()> {
  let config = Config::from_args(std::env::args().collect())?;

  for e in config.requests.endpoints.iter() {
    // For now if error happened just skip the request and proceed
    // to the next one
    if let Ok(response) = make_http_request(&config, &e) {
      write_response_to_file(&config, response, &e.url)
    };
  }

  Ok(())
}

fn make_http_request(config: &Config, endpoint: &Endpoint) -> Result<String> {
  let client = reqwest::blocking::Client::builder()
      .default_headers(headers::create_header_map(&config, &endpoint))
      .build()
      .unwrap();

  // TODO: Handle unwraps here
  Ok(client.get(&endpoint.url).send().unwrap().text().unwrap())
}

fn write_response_to_file(config: &Config, response: String, url: &String) {
  use std::fs;
  use std::fs::File;
  use std::io::Write;
  use reqwest::Url;

  // TODO: Handle unwraps here
  let url = Url::parse(&url).unwrap();
  let url_path: Vec<&str> = url.path_segments().unwrap().collect();

  let (file_name, url_path) = url_path.split_last().unwrap();

  let url_path = format!("{}/{}", config.dst, url_path.join("/"));
  let file_name = format!("{}/{}.{}", url_path, file_name, config::CONFIG_FILE_EXT);

  fs::create_dir_all(url_path);

  if let Ok(mut f) = File::create(file_name) {
    f.write_all(&response.as_bytes());
  }
}
