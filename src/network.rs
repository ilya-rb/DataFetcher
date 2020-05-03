use crate::config::Config;
use crate::config::Endpoint;
use crate::errors::AppErrorType;
use crate::errors::Error::AppError;
use crate::types::Result;

use std::collections::HashMap;
use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub fn make_http_request(config: &Config, endpoint: &Endpoint) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let result = client
        .request(
            endpoint.method.clone(),
            reqwest::Url::parse(&endpoint.url).unwrap(),
        )
        .headers(create_header_map(&config, &endpoint))
        .send()
        .map_err(|_| {
            AppError(
                AppErrorType::RequestFetchError,
                "Error performing http request",
            )
        })?;

    let status = result.status();
    if status.is_success() {
        info!("{}", status);
    } else {
        error!("{}", status);
    }

    Ok(result.text().map_err(|_| {
        AppError(
            AppErrorType::ResponseParseError,
            "Error parsing request body",
        )
    })?)
}

fn create_header_map(config: &Config, endpoint: &Endpoint) -> HeaderMap {
    let global = create_headers(&config.requests.headers);
    let child = create_headers(&endpoint.headers);
    global.into_iter().chain(child).collect()
}

fn create_headers(headers: &HashMap<String, String>) -> HashMap<HeaderName, HeaderValue> {
    let mut result = HashMap::new();

    for (k, v) in headers.iter() {
        let name = HeaderName::from_str(k);
        let value = HeaderValue::from_str(v);

        // Filter invalid headers
        if name.is_err() || value.is_err() {
            warn!(
                "Invalid request header:\nName :: {:?}\nValue :: {:?}",
                name, value
            );
            continue;
        }

        result.insert(name.unwrap(), value.unwrap());
    }

    result
}
