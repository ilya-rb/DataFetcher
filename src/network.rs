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
        .headers(create_headers(&config.requests.headers))
        .headers(create_headers(&endpoint.headers))
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

fn create_headers(headers: &HashMap<String, String>) -> HeaderMap {
    let mut result = HeaderMap::new();

    headers
        .iter()
        .map(|(k, v)| (HeaderName::from_str(k), HeaderValue::from_str(v)))
        .inspect(|(k, v)| {
            if k.is_err() || v.is_err() {
                warn!(
                    "Invalid request header:\nName :: {:?}\nValue :: {:?}, skipping",
                    k, v
                );
            }
        })
        .filter(|(k, v)| k.is_ok() && v.is_ok())
        .for_each(|(k, v)| {
            result.insert(k.unwrap(), v.unwrap());
            ()
        });

    result
}
