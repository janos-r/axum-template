use crate::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: set real token
    cookies.add(Cookie::new(super::AUTH_TOKEN, "user-1.exp.sign"));
    // TODO: test hello before and after login
    //     find cookie in login set-cookie header, response cookie, client cookie (login and hello)

    let body = Json(json!({
        "result:": {
            "success": true
        }
    }));

    Ok(body)
}
