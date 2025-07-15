use std::error::Error;

use serde::Deserialize;
use snix_eval::Value;

use super::http_request;

#[derive(Deserialize)]
pub struct Params {
    pub response: Response,
    #[serde(default)]
    pub assertions: Vec<bool>,
}

#[derive(Deserialize)]
pub struct Response {
    pub request: http_request::Params,
    pub status: Option<u16>,
    pub body: Option<String>,
    #[serde(rename = "json")]
    pub _json: Option<Value>,
}

pub async fn handler(params: &Params) -> Result<(), Box<dyn Error>> {
    if params.response.status.is_none()
        && let Some(message) = params.response.body.as_ref()
    {
        Err(format!("Request failed ( {message} )").into())
    } else if !params.assertions.is_empty() && params.assertions.iter().any(|assertion| !assertion)
    {
        let result = params
            .assertions
            .iter()
            .map(|assertion| if *assertion { "✅ " } else { "❌ " })
            .collect::<String>();

        Err(format!("Assertions failed ( {result})").into())
    } else {
        Ok(())
    }
}
