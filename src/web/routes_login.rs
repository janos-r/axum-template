use crate::mw_ctx::{CtxState, JWT_AUTH, JWT_KEY};
use crate::{ctx::Ctx, error::ApiError, error::Error, ApiResult};
use axum::extract::State;
use axum::{routing::post, Json, Router};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tower_cookies::{Cookie, Cookies};

pub fn routes(state: CtxState) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct LoginInput {
    email: String,
    password: String,
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
    State(CtxState { _db, key }): State<CtxState>,
    cookies: Cookies,
    ctx: Ctx,
    payload: Json<LoginInput>,
) -> ApiResult<Json<LoginResult>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // NOTE: DB should be checked here
    // Mock user
    struct User {
        email: String,
        password: String,
    }
    let mock_user = User {
        email: "joe@example.com".to_string(),
        password: "123".to_string(),
    };

    if payload.email != mock_user.email || payload.password != mock_user.password {
        return Err(ApiError {
            error: Error::LoginFail,
            req_id: ctx.req_id(),
        });
    };

    let mut claims = BTreeMap::new();
    claims.insert(JWT_AUTH, mock_user.email);
    // TODO: don't know how to set expiration
    let token_str = claims.sign_with_key(&key).unwrap();

    cookies.add(
        Cookie::build(JWT_KEY, token_str)
            // if not set, the path defaults to the path from which it was called - prohibiting gql on root if login is on /api
            .path("/")
            .finish(),
    );

    Ok(Json(LoginResult {
        result: LoginSuccess { success: true },
    }))
}
