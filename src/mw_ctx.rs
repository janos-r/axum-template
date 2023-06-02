use axum::{
    extract::{FromRequestParts, State},
    http::Request,
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    error::{ApiError, Result},
    model::ModelController,
    ApiResult, Error,
};

pub const AUTH_TOKEN: &str = "auth-token";

pub async fn mw_require_auth<B>(ctx: Ctx, req: Request<B>, next: Next<B>) -> ApiResult<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    ctx.user_id()?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_constructor<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    println!("->> {:<12} - mw_ctx_constructor", "MIDDLEWARE");

    let uuid = Uuid::new_v4();
    let result_user_id: Result<u64> = match extract_token(&cookies) {
        Ok((user_id, _exp, _sign)) => Ok(user_id),
        Err(err) => {
            // Remove a wrongly formated cookie
            if err == Error::AuthFailTokenWrongFormat {
                cookies.remove(Cookie::named(AUTH_TOKEN))
            }
            Err(err)
        }
    };
    // TODO: token validation with DB

    // Store Ctx in the request extension
    let ctx = Ctx::new(result_user_id, uuid);
    req.extensions_mut().insert(ctx);

    next.run(req).await
}

type Token = (u64, String, String);
fn parse_token(token: &str) -> Result<Token> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d)\.(.+)\.(.+)"#, token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_owned(), sign.to_owned()))
}
fn extract_token(cookies: &Cookies) -> Result<Token> {
    cookies
        .get(AUTH_TOKEN)
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(|c| parse_token(c.value()))
}

// ugly but direct implementation from axum, until "async trait fn" are in stable rust, instead of importing some 3rd party macro
// Extractor - makes it possible to specify Ctx as a param - fetches the result from the header parts extension
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = ApiError;
    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut axum::http::request::Parts,
        _state: &'life1 S,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ApiResult<Self>> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            println!(
                "->> {:<12} - Ctx::from_request_parts - extract Ctx from extension",
                "EXTRACTOR"
            );
            parts.extensions.get::<Ctx>().cloned().ok_or(ApiError {
                req_id: Uuid::new_v4(),
                error: Error::AuthFailCtxNotInRequestExt,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_token() {
        let (id, exp, sign) = super::parse_token("user-1.exp.sign").unwrap();
        assert_eq!(id, 1);
        assert_eq!(exp, "exp");
        assert_eq!(sign, "sign");
    }
}
