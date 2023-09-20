use crate::mw_ctx::{Claims, CtxState, JWT_KEY};
use crate::{ctx::Ctx, error::ApiError, error::Error, ApiResult};
use axum::extract::State;
use axum::{routing::post, Json, Router};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
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
    State(CtxState { _db, key_enc, .. }): State<CtxState>,
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

    // NOTE: set to a reasonable number after testing
    // NOTE when testing: the default validation.leeway is 2min
    let exp = Utc::now() + Duration::minutes(2);
    let claims = Claims {
        exp: exp.timestamp() as usize,
        auth: mock_user.email,
    };
    let token_str = encode(&Header::default(), &claims, &key_enc).expect("JWT encode should work");

    cookies.add(
        Cookie::build(JWT_KEY, token_str)
            // if not set, the path defaults to the path from which it was called - prohibiting gql on root if login is on /api
            .path("/")
            .http_only(true)
            .finish(),
    );

    Ok(Json(LoginResult {
        result: LoginSuccess { success: true },
    }))
}
