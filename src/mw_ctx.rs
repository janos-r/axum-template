use crate::{ctx::Ctx, error::Error, error::Result, ApiResult, Db};
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

#[derive(Clone)]
pub struct CtxState {
    // NOTE: with DB, because a real login would check the DB
    pub _db: Db,
    pub key_enc: EncodingKey,
    pub key_dec: DecodingKey,
}

pub const JWT_KEY: &str = "jwt";
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub auth: String,
}

pub async fn mw_require_auth<B>(ctx: Ctx, req: Request<B>, next: Next<B>) -> ApiResult<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    ctx.user_id()?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_constructor<B>(
    State(CtxState { _db, key_dec, .. }): State<CtxState>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    println!("->> {:<12} - mw_ctx_constructor", "MIDDLEWARE");

    let uuid = Uuid::new_v4();
    let result_user_id: Result<String> = extract_token(key_dec, &cookies).map_err(|err| {
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

fn verify_token(key: DecodingKey, token: &str) -> Result<String> {
    Ok(decode::<Claims>(token, &key, &Validation::default())?
        .claims
        .auth)
}
fn extract_token(key: DecodingKey, cookies: &Cookies) -> Result<String> {
    cookies
        .get(JWT_KEY)
        .ok_or(Error::AuthFailNoJwtCookie)
        .and_then(|cookie| verify_token(key, cookie.value()))
}

#[cfg(test)]
mod tests {
    use crate::mw_ctx::Claims;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{
        decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
    };

    const SECRET: &[u8] = b"some-secret";
    const SOMEONE: &str = "someone";
    // cspell:disable-next-line
    const TOKEN_EXPIRED: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjEsImF1dGgiOiJzb21lb25lIn0.XXHVHu2IsUPA175aQ-noWbQK4Wu-2prk3qTXjwaWBvE";

    #[test]
    fn jwt_sign_expired() {
        let my_claims = Claims {
            exp: 1,
            auth: SOMEONE.to_string(),
        };
        let token_str = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();
        assert_eq!(token_str, TOKEN_EXPIRED);
    }

    #[test]
    fn jwt_verify_expired_ignore() {
        let mut validation = Validation::default();
        validation.validate_exp = false;
        let token = decode::<Claims>(
            TOKEN_EXPIRED,
            &DecodingKey::from_secret(SECRET),
            &validation,
        )
        .unwrap();
        assert_eq!(token.claims.auth, SOMEONE);
    }

    #[test]
    fn jwt_verify_expired_fail() {
        let token_result = decode::<Claims>(
            TOKEN_EXPIRED,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        );
        assert!(token_result.is_err());
        let kind = token_result.map_err(|e| e.into_kind()).err();
        assert_eq!(kind, Some(ErrorKind::ExpiredSignature));
    }

    #[test]
    fn jwt_sign_and_verify_with_chrono() {
        let exp = Utc::now() + Duration::minutes(1);
        let my_claims = Claims {
            exp: exp.timestamp() as usize,
            auth: SOMEONE.to_string(),
        };
        // Sign
        let token_str = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();
        // Verify
        let token_result = decode::<Claims>(
            &token_str,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )
        .unwrap();
        assert_eq!(token_result.claims.auth, SOMEONE);
    }
}
