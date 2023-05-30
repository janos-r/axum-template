use axum::{
    extract::FromRequestParts, http::Request, middleware::Next, response::Response, RequestPartsExt,
};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use super::AUTH_TOKEN;
use crate::{ctx::Ctx, Error, Result};

pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    ctx?;
    Ok(next.run(req).await)
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
fn extract_token(cookies: Cookies) -> Result<Token> {
    cookies
        .get(AUTH_TOKEN)
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(|c| parse_token(c.value()))
    // TODO: token validation
}

// ugly but direct implementation from axum, until "async trait fn" are in stable rust, instead of importing some 3rd party macro
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut axum::http::request::Parts,
        _state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = std::result::Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            println!("->> {:<12} - Ctx::from_request_parts", "EXTRACTOR");
            let cookies = parts.extract::<Cookies>().await.unwrap();
            let (user_id, exp, sign) = extract_token(cookies)?;
            Ok(Ctx::new(user_id))
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
