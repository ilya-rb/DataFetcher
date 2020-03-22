use crate::config::Config;
use crate::config::Endpoint;
use crate::errors::Error;

use crate::types::Result;

use std::collections::HashMap;
use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn make_http_request(config: &Config, endpoint: &Endpoint) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .default_headers(create_header_map(&config, &endpoint))
        .build()
        .map_err(|_| Error::unknown_error())?;

    // TODO: Map this to concrete errors
    let result = client
        .get(&endpoint.url)
        .send()
        .map_err(|_| Error::unknown_error())?
        .text()
        .map_err(|_| Error::unknown_error())?;

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
