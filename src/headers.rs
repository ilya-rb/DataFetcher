use crate::config::{Config, Endpoint};

use reqwest::header::{
  HeaderName,
  HeaderMap,
  HeaderValue,
};

pub fn create_header_map(config: &Config, endpoint: &Endpoint) -> HeaderMap {
  use std::collections::HashMap;

  let glob_headers = config.requests.as_ref().unwrap().headers.as_ref().unwrap();
  let glob_headers: HashMap<HeaderName, HeaderValue> = glob_headers.into_iter()
      .map(|(name, value)| create_header(&name, &value))
      .collect();

  let request_headers: HashMap<HeaderName, HeaderValue> = endpoint.headers
      .as_ref()
      .unwrap_or(&HashMap::new())
      .into_iter()
      .map(|(name, value)| create_header(name, value))
      .collect();

  request_headers.into_iter()
      .chain(glob_headers)
      .collect()
}

fn create_header(name: &String, value: &String) -> (HeaderName, HeaderValue) {
  (
    HeaderName::from_bytes(name.as_bytes()).unwrap(),
    HeaderValue::from_bytes(value.as_bytes()).unwrap()
  )
}