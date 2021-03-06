use crate::config::Config;
use crate::config::Endpoint;
use crate::errors::AppErrorType;
use crate::errors::Error::AppError;
use crate::files;
use crate::types::Result;

use std::collections::HashMap;
use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub enum ContentType {
    Json,
    Text,
    Html,
}

impl ContentType {
    pub fn get_file_extension(&self) -> &str {
        match self {
            ContentType::Json => files::EXT_JSON,
            ContentType::Text => files::EXT_TEXT,
            ContentType::Html => files::EXT_HTML,
        }
    }

    fn from_header_value(header_value: &HeaderValue) -> ContentType {
        if header_value == CONTENT_TYPE_JSON {
            ContentType::Json
        } else if header_value == CONTENT_TYPE_HTML {
            ContentType::Html
        } else {
            ContentType::Text
        }
    }
}

const CONTENT_TYPE_JSON: &str = "application/json";
const CONTENT_TYPE_HTML: &str = "text/html";

pub struct Response {
    pub content_type: ContentType,
    pub response_text: String,
}

pub fn make_http_request(config: &Config, endpoint: &Endpoint) -> Result<Response> {
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

    let content_type = match result.headers().get(reqwest::header::CONTENT_TYPE) {
        Some(content_type) => ContentType::from_header_value(content_type),
        None => ContentType::Text,
    };

    let response_text = result.text().map_err(|_| {
        AppError(
            AppErrorType::ResponseParseError,
            "Error parsing request body",
        )
    })?;

    Ok(Response {
        content_type,
        response_text,
    })
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
        });

    result
}
