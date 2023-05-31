use axum::{http::StatusCode, response::IntoResponse};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    _Generic { description: &'static str },
    LoginFail,
    TicketDeleteFailIdNotFound { id: u64 },
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

pub type Result<T> = core::result::Result<T, Error>;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::_Generic { description } => write!(f, "{description}"),
            Self::LoginFail => write!(f, "Login fail"),
            Self::TicketDeleteFailIdNotFound { id } => write!(f, "Ticket id {id} not found"),
            Self::AuthFailNoAuthTokenCookie => write!(f, "You are not logged in"),
            Self::AuthFailTokenWrongFormat => {
                write!(f, "Can't parse token, wrong format")
            }
            Self::AuthFailCtxNotInRequestExt => write!(f, "Internal error"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - into_response - {self:?}", "ERROR");
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn display_description() {
        let err = super::Error::_Generic {
            description: "super description",
        };
        assert_eq!(format!("{err}"), "super description");
        assert_eq!(err.to_string(), "super description");
    }
}
