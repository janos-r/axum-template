use crate::{error::ApiError, ApiResult, Error};
use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginInput {
    username: String,
    pwd: String,
}
#[derive(Debug, Serialize)]
struct LoginSuccess {
    success: bool,
}
#[derive(Debug, Serialize)]
struct LoginResult {
    result: LoginSuccess,
}

async fn api_login(cookies: Cookies, payload: Json<LoginInput>) -> ApiResult<Json<LoginResult>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(ApiError {
            error: Error::LoginFail,
            // TODO: fix - extract from ctx
            uuid: Uuid::new_v4(),
        });
    };

    // TODO: set real token
    cookies.add(Cookie::new(super::AUTH_TOKEN, "user-1.exp.sign"));

    Ok(Json(LoginResult {
        result: LoginSuccess { success: true },
    }))
}
