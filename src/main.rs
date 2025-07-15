use snix_eval::EvalMode;
use snix_eval::builtin_macros::builtins;

fn main() {
    let code = std::fs::read_to_string("tests.nix").expect("Failed to read tests.nix");

    let evaluation = snix_eval::Evaluation::builder_impure()
        .add_builtins(extra::builtins())
        .mode(EvalMode::Strict)
        .disable_import()
        .build();

    let eval = evaluation.evaluate(code, None);

    if !eval.errors.is_empty() {
        for error in eval.errors {
            println!("Error: {error:?}");
        }
        return;
    }

    if !eval.warnings.is_empty() {
        for warning in eval.warnings {
            println!("Warning: {warning:?}");
        }
    }
}

#[builtins]
mod extra {
    use std::collections::BTreeMap;
    use std::str::FromStr;
    use std::time::Duration;

    use reqwest::Method;
    use reqwest::blocking::Client;
    use serde::Deserialize;
    use serde_json::{
        from_str,
        from_value,
    };
    use snix_eval::generators::{
        Gen,
        GenCo,
    };
    use snix_eval::{
        ErrorKind,
        NixAttrs,
        NixString,
        Value,
    };

    #[derive(Debug, Deserialize)]
    struct Params {
        method: String,
        url: String,
        #[serde(default)]
        headers: BTreeMap<String, String>,
        #[serde(default)]
        timeout: Option<u64>,
    }

    #[builtin("httpRequest")]
    pub async fn http_request(
        co: GenCo,
        args: Value,
    ) -> Result<Value, ErrorKind> {
        let (json, _ctx) = args.into_contextful_json(&co).await?;

        let Params {
            method,
            url,
            headers,
            timeout,
        } = from_value(json).expect("Failed to parse parameters");

        let method = Method::from_str(&method).expect("Failed to parse method");

        let mut http_client_builder = Client::builder();

        if let Some(timeout) = timeout {
            http_client_builder = http_client_builder.timeout(Duration::from_millis(timeout));
        }

        let http_client = http_client_builder
            .build()
            .expect("Failed to build http client");

        let mut request = http_client.request(method, url);

        for (key, value) in headers {
            request = request.header(key, value)
        }

        let response = request.send().expect("Failed to send http request");

        let status = response.status().as_u16();

        let bytes = response.bytes().expect("Failed to read response body");

        let body = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        // snix_eval::Value instead of serde_json::Value but we can safely assume that
        // if the response body is a valid JSON string, it can be parsed into a
        // snix_eval::Value
        let json: Value = from_str(&body).unwrap_or(Value::Null);

        Ok(Value::attrs(NixAttrs::from_iter([
            ("status", Value::Integer(i64::from(status))),
            ("body", Value::String(NixString::from(body))),
            ("json", json),
        ])))
    }
}
