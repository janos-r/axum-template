use axum::{http::StatusCode, response::IntoResponse};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    _Generic { description: &'static str },
    LoginFail,
}

pub type Result<T> = core::result::Result<T, Error>;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::_Generic { description } => write!(f, "{description}"),
            Self::LoginFail => write!(f, "login fail"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - into_response - {self}", "ERROR");
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
