use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiError {
    pub error: Error,
    pub req_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    _Generic { description: &'static str },
    LoginFail,
    TicketDeleteFailIdNotFound { id: u64 },
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

pub type ApiResult<T> = core::result::Result<T, ApiError>;
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

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - into_response - {self:?}", "ERROR");
        let status_code = match self.error {
            Error::_Generic { .. } | Error::LoginFail => StatusCode::FORBIDDEN,
            Error::AuthFailNoAuthTokenCookie
            | Error::AuthFailTokenWrongFormat
            | Error::AuthFailCtxNotInRequestExt => StatusCode::FORBIDDEN,
            Error::TicketDeleteFailIdNotFound { .. } => StatusCode::BAD_REQUEST,
        };
        let body = Json(json!({
            "error": {
                "error": self.error.to_string(),
                "uuid": self.req_id.to_string()
            }
        }));
        let mut response = (status_code, body).into_response();
        // Insert the Error into the response - for the logger
        response.extensions_mut().insert(self.error);
        response
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
