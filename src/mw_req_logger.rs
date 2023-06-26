use crate::{ctx::Ctx, error::Error};
use axum::{
    http::{Method, Uri},
    response::Response,
};
use serde::Serialize;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug)]
struct RequestLog {
    req_method: String,
    req_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    timestamp: String,
    req_id: String,
}

pub async fn mw_req_logger(ctx: Ctx, uri: Uri, req_method: Method, res: Response) -> Response {
    let log = RequestLog {
        req_id: ctx.req_id().to_string(),
        user: ctx.user_id().ok(),
        error: res.extensions().get::<Error>().map(|e| format!("{e:?}")),
        req_path: uri.to_string(),
        req_method: req_method.to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string(),
    };
    println!("->> {:<12} - mw_req_logger:", "LOGGER",);
    println!("{:4}{}\n", "", json!(log));
    res
}
