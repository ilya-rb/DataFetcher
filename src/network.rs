use crate::config::Config;
use crate::config::Endpoint;
use crate::types::Result;

use std::collections::HashMap;
use std::str::FromStr;

use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

pub fn make_http_request(config: &Config, endpoint: &Endpoint) -> Result<String> {
  use crate::errors::AppErrorType;
  use crate::errors::Error::AppError;

  use AppErrorType::UnableToCreateHttpClient;
  use AppErrorType::RequestFetchError;
  use AppErrorType::ResponseParseError;

  let client = reqwest::blocking::Client::builder()
    .default_headers(create_header_map(&config, &endpoint))
    .build()
    .map_err(|_| AppError(UnableToCreateHttpClient, "Unable to create http client"))?;

let result = client
  .get(&endpoint.url)
  .send()
  .map_err(|_| AppError(RequestFetchError, "Error performing http request"))?
  .text()
  .map_err(|_| AppError(ResponseParseError, "Error parsing request body"))?;

  Ok(result)
}

fn create_header_map(config: &Config, endpoint: &Endpoint) -> HeaderMap {
  create_headers(config.requests.headers.as_ref())
  .into_iter()
  .chain(create_headers(endpoint.headers.as_ref()))
  .collect()
}

fn create_headers(headers: Option<&HashMap<String, String>>) -> HashMap<HeaderName, HeaderValue> {
  headers
  .unwrap_or(&HashMap::new())
  .iter()
  .map(|(name, value)| create_header(&name, &value))
  .collect()
}

fn create_header(name: &str, value: &str) -> (HeaderName, HeaderValue) {
  (
    HeaderName::from_str(name).unwrap(),
    HeaderValue::from_str(value).unwrap(),
  )
}
