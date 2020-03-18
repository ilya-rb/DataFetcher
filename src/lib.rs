mod config;
mod errors;
mod headers;

use config::Config;
use config::Endpoint;
use errors::Error;

pub fn run() -> Result<(), Error> {
  let config = Config::new()?;
  config.validate()?;
  perform_requests(config)?;
  Ok(())
}

fn perform_requests(config: Config) -> Result<(), Error> {
  let endpoints = config.requests.as_ref().unwrap().endpoints.as_ref().unwrap();

  for e in endpoints.iter() {
    let response = make_http_request(&config, &e)?;
    store_response_to_file(&config, response, &e.url);
  }

  Ok(())
}

fn make_http_request(config: &Config, endpoint: &Endpoint) -> Result<String, Error> {
  let client = reqwest::blocking::Client::builder()
      .default_headers(headers::create_header_map(&config, &endpoint))
      .build()
      .unwrap();

  let response: reqwest::blocking::Response = client.get(&endpoint.url).send().unwrap();
  Ok(response.text().unwrap())
}

fn store_response_to_file(config: &Config, response: String, url: &String) {
  use std::fs;
  use std::fs::File;
  use std::io::Write;
  use reqwest::Url;

  let url = Url::parse(&url).unwrap();
  let url_path: Vec<&str> = url.path_segments().unwrap().collect();

  let (file_name, url_path) = url_path.split_last().unwrap();

  let url_path = format!("{}/{}", config.dst.as_ref().unwrap(), url_path.join("/"));
  let file_name = format!("{}/{}.{}", url_path, file_name, config::CONFIG_FILE_EXT);

  fs::create_dir_all(url_path);

  if let Ok(mut f) = File::create(file_name) {
    f.write_all(&response.as_bytes());
  }
}
