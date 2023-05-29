use axum::{http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use super::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let (user_id, exp, sign) = cookies
        .get(AUTH_TOKEN)
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(|c| parse_token(c.value()))?;

    // TODO: token validation

    Ok(next.run(req).await)
}

pub fn parse_token(token: &str) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d)\.(.+)\.(.+)"#, token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_owned(), sign.to_owned()))
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
