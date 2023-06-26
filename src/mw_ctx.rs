use crate::{ctx::Ctx, error::Error, error::Result, ApiResult, Db};
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use hmac::Hmac;
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

#[derive(Clone)]
pub struct CtxState {
    // NOTE: with DB, because a real login would check the DB
    pub _db: Db,
    pub key: Hmac<Sha256>,
}

pub const JWT_KEY: &str = "jwt";
pub const JWT_AUTH: &str = "auth";

pub async fn mw_require_auth<B>(ctx: Ctx, req: Request<B>, next: Next<B>) -> ApiResult<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    ctx.user_id()?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_constructor<B>(
    State(CtxState { _db, key }): State<CtxState>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    println!("->> {:<12} - mw_ctx_constructor", "MIDDLEWARE");

    let uuid = Uuid::new_v4();
    let result_user_id: Result<String> = extract_token(key, &cookies).map_err(|err| {
        // Remove an invalid cookie
        if let Error::AuthFailJwtInvalid { .. } = err {
            cookies.remove(Cookie::named(JWT_KEY))
        }
        err
    });
    // NOTE: DB should be checked here

    // Store Ctx in the request extension, for extracting in rest handlers
    let ctx = Ctx::new(result_user_id, uuid);
    req.extensions_mut().insert(ctx);

    next.run(req).await
}

fn verify_token(key: Hmac<Sha256>, token: &str) -> Result<String> {
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    claims
        .get(JWT_AUTH)
        .ok_or(Error::AuthFailJwtWithoutAuth)
        .map(String::from)
}
fn extract_token(key: Hmac<Sha256>, cookies: &Cookies) -> Result<String> {
    cookies
        .get(JWT_KEY)
        .ok_or(Error::AuthFailNoJwtCookie)
        .and_then(|cookie| verify_token(key, cookie.value()))
}

#[cfg(test)]
mod tests {
    use crate::mw_ctx::JWT_AUTH;
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use sha2::Sha256;
    use std::collections::BTreeMap;

    const SECRET: &[u8] = b"some-secret";
    const SOMEONE: &str = "someone";
    const TOKEN: &str =
    // cspell:disable-next-line
        "eyJhbGciOiJIUzI1NiJ9.eyJhdXRoIjoic29tZW9uZSJ9.1g78DkCARXRPLRlRbzv_nKZZuykVr5_nwPaifpVTvvM";

    #[test]
    fn jwt_sign() {
        let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert(JWT_AUTH, SOMEONE);
        let token_str = claims.sign_with_key(&key).unwrap();
        assert_eq!(token_str, TOKEN);
    }

    #[test]
    fn jwt_verify() {
        let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET).unwrap();
        let user_id = super::verify_token(key, TOKEN).unwrap();
        assert_eq!(user_id, SOMEONE);
    }
}
