use crate::config::{Config, Endpoint, Requests};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct YamlConfig {
    pub dst: String,
    pub requests: YamlRequests,
}

impl YamlConfig {
    pub fn as_config(&self) -> Config {
        Config {
            dst: self.dst.to_owned(),
            requests: self.requests.as_requests(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct YamlRequests {
    pub headers: Option<Vec<String>>,
    pub endpoints: Option<Vec<YamlEndpoint>>,
}

impl YamlRequests {
    fn as_requests(&self) -> Requests {
        Requests {
            headers: convert_headers_to_map(self.headers.as_ref().unwrap_or(&Vec::new())),
            endpoints: self
                .endpoints
                .as_ref()
                .map_or(Vec::new(), |e| e.iter().map(|e| e.as_endpoint()).collect()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct YamlEndpoint {
    pub url: String,
    pub headers: Option<Vec<String>>,
}

impl YamlEndpoint {
    fn as_endpoint(&self) -> Endpoint {
        let components: Vec<&str> = self.url.split(' ').collect();
        let (method, url) = if components.len() == 2 {
            (components[0], components[1])
        } else {
            // Default method for requests will be get to reduce
            // some boilerplate
            ("GET", components[0])
        };

        Endpoint {
            url: url.to_owned(),
            headers: convert_headers_to_map(self.headers.as_ref().unwrap_or(&Vec::new())),
            method: parse_method(method),
        }
    }
}

fn parse_method(method: &str) -> reqwest::Method {
    use reqwest::Method;
    Method::from_bytes(method.to_lowercase().as_bytes()).unwrap_or(Method::GET)
}

fn convert_headers_to_map(headers: &Vec<String>) -> HashMap<String, String> {
    headers
        .iter()
        .map(|h| h.split(':').collect::<Vec<&str>>())
        .filter(|h| h.len() == 2)
        .map(|h| (h[0].to_string(), h[1].to_string()))
        .collect()
}
