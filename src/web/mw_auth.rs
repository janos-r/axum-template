use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use super::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_owned())
        .ok_or(Error::AuthFailNoAuthTokenCookie)?;

    // TODO: token validation

    Ok(next.run(req).await)
}
