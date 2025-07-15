use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

use reqwest::Method;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    method: String,
    url: String,
    #[serde(default)]
    headers: BTreeMap<String, String>,
    #[serde(default)]
    timeout: Option<u64>,
}

pub async fn http_request(params: Params) -> Result<(u16, Option<String>), Box<dyn Error>> {
    let method = Method::from_str(&params.method)
        .map_err(|_| format!("Invalid method: {}", params.method))?;

    let mut http_client_builder = Client::builder();

    if let Some(timeout) = params.timeout {
        http_client_builder = http_client_builder.timeout(Duration::from_millis(timeout));
    }

    let http_client = http_client_builder
        .build()
        .expect("Failed to build http client");

    let mut request = http_client.request(method, params.url);

    for (key, value) in params.headers {
        request = request.header(key, value)
    }

    let response = request
        .send()
        .map_err(|err| format!("Failed to send http request: {err}"))?;

    let status = response.status().as_u16();

    let body: Option<String> = response
        .bytes()
        .ok()
        .map(|bytes| String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string"));

    Ok((status, body))
}
