use crate::config::{Config, Endpoint};
use crate::types::Result;

use std::str::FromStr;
use std::collections::HashMap;

use reqwest::header::{
  HeaderName,
  HeaderMap,
  HeaderValue,
};

pub fn create_header_map(config: &Config, endpoint: &Endpoint) -> HeaderMap {
  create_headers(config.requests.headers.as_ref())
      .into_iter()
      .chain(create_headers(endpoint.headers.as_ref()))
      .collect()
}

fn create_headers(headers: Option<&HashMap<String, String>>) -> HashMap<HeaderName, HeaderValue> {
  headers
      .unwrap_or(&HashMap::new())
      .into_iter()
      .map(|(name, value)| create_header(&name, &value))
      .collect()
}

fn create_header(name: &String, value: &String) -> (HeaderName, HeaderValue) {
  (
    HeaderName::from_str(name).unwrap(),
    HeaderValue::from_str(value).unwrap()
  )
}