mod http_request;
mod test;

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

    #[builtin("test")]
    pub async fn builtins_test(
        co: GenCo,
        args: Value,
    ) -> Result<Value, ErrorKind> {
        use super::test::{
            Params,
            handler,
        };

        let (json, _ctx) = args.clone().into_contextful_json(&co).await?;

        let params: Params = from_value(json).expect("Failed to parse parameters");

        let method = &params.response.request.method;
        let url = &params.response.request.url;

        match handler(&params).await {
            Ok(_) => println!("✅ {method} {url}"),
            Err(err) => println!("❌ {method} {url}: {err}"),
        }

        Ok(args)
    }

    #[builtin("httpRequest")]
    pub async fn builtins_http_request(
        co: GenCo,
        args: Value,
    ) -> Result<Value, ErrorKind> {
        use super::http_request::{
            Params,
            handler,
        };

        let (json, _ctx) = args.clone().into_contextful_json(&co).await?;

        let params: Params = from_value(json).expect("Failed to parse parameters");

        let (status, body) = match handler(&params).await {
            Ok(res) => res,
            Err(err) => {
                let message = Value::String(NixString::from(err.to_string()));

                return Ok(Value::attrs(NixAttrs::from_iter([
                    ("status", Value::Null),
                    ("body", message),
                    ("json", Value::Null),
                ])));
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
            ("request", args),
            ("status", status),
            ("body", body),
            ("json", json),
        ])))
    }
}

pub use extra::builtins;
