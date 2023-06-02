use crate::{ctx::Ctx, error::ApiError, ApiResult, Error};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

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

async fn api_login(
    cookies: Cookies,
    ctx: Ctx,
    payload: Json<LoginInput>,
) -> ApiResult<Json<LoginResult>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(ApiError {
            error: Error::LoginFail,
            req_id: ctx.req_id(),
        });
    };

    // TODO: set real token
    cookies.add(Cookie::new(crate::mw_ctx::AUTH_TOKEN, "user-1.exp.sign"));

    Ok(Json(LoginResult {
        result: LoginSuccess { success: true },
    }))
}
