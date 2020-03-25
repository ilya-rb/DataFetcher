use crate::config::Config;
use crate::config::Endpoint;
use crate::types::Result;
use crate::errors::AppErrorType;
use crate::errors::Error::AppError;

use std::collections::HashMap;
use std::str::FromStr;

use reqwest::header::{
  HeaderMap,
  HeaderName,
  HeaderValue,
};

pub fn make_http_request(config: &Config, endpoint: &Endpoint) -> Result<String> {
  let client = reqwest::blocking::Client::builder()
    .default_headers(create_header_map(&config, &endpoint))
    .build()
    .map_err(|_| AppError(AppErrorType::UnableToCreateHttpClient, "Unable to create http client"))?;

    let result = client
      .get(&endpoint.url)
      .send();

    if result.is_err() {
      return Err(AppError(AppErrorType::RequestFetchError, "Error performing http request"));
    }

    let text = result.unwrap().text();
    if text.is_err() {
      return Err(AppError(AppErrorType::ResponseParseError, "Error parsing request body"));
    }

    Ok(text.unwrap())
}

fn create_header_map(config: &Config, endpoint: &Endpoint) -> HeaderMap {
  let glob_headers = create_headers(config.requests.headers.as_ref());
  let child_headers = create_headers(endpoint.headers.as_ref());
  
  glob_headers.into_iter()
    .chain(child_headers)
    .collect()
}

fn create_headers(headers: Option<&HashMap<String, String>>) -> HashMap<HeaderName, HeaderValue> {
  let mut result = HashMap::new();

  if let Some(headers) = headers.as_ref() {
    for (k, v) in headers.iter() {
      let name = HeaderName::from_str(k);
      let value = HeaderValue::from_str(v);

      // Filter invalid headers
      if name.is_err() || value.is_err() {
        warn!("Invalid request header:\nName :: {:?}\nValue :: {:?}", name, value);
        continue;
      }

      result.insert(name.unwrap(), value.unwrap());
    }
  };

  result
}
