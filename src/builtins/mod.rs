mod http_request;

use snix_eval::builtin_macros::builtins;

#[builtins]
mod extra {
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

    use super::http_request::{
        Params,
        http_request,
    };

    #[builtin("httpRequest")]
    pub async fn builtins_http_request(
        co: GenCo,
        args: Value,
    ) -> Result<Value, ErrorKind> {
        let (json, _ctx) = args.into_contextful_json(&co).await?;

        let params: Params = from_value(json).expect("Failed to parse parameters");

        let (status, body) = match http_request(params).await {
            Ok(res) => res,
            Err(err) => {
                println!("Error: {err}");
                return Ok(Value::Null);
            }
        };

        let status = Value::Integer(status as i64);

        // snix_eval::Value instead of serde_json::Value but we can safely assume that
        // if the response body is a valid JSON string, it can be parsed into a
        // snix_eval::Value
        let json: Value = body
            .as_ref()
            .and_then(|body| from_str(body).ok())
            .unwrap_or(Value::Null);

        let body: Value = match body {
            Some(body) => Value::String(NixString::from(body)),
            None => Value::Null,
        };

        Ok(Value::attrs(NixAttrs::from_iter([
            ("status", status),
            ("body", body),
            ("json", json),
        ])))
    }
}

pub use extra::builtins;
