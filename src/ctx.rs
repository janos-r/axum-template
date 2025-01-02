use crate::error::*;
use axum::extract::FromRequestParts;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Ctx {
    result_user_id: Result<String>,
    req_id: Uuid,
}

impl Ctx {
    pub fn new(result_user_id: Result<String>, uuid: Uuid) -> Self {
        Self {
            result_user_id,
            req_id: uuid,
        }
    }

    pub fn user_id(&self) -> ApiResult<String> {
        self.result_user_id.clone().map_err(|error| ApiError {
            error,
            req_id: self.req_id,
        })
    }

    pub fn req_id(&self) -> Uuid {
        self.req_id
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> ApiResult<Self> {
        println!(
            "->> {:<12} - Ctx::from_request_parts - extract Ctx from extension",
            "EXTRACTOR"
        );
        parts.extensions.get::<Ctx>().cloned().ok_or(ApiError {
            req_id: Uuid::new_v4(),
            error: Error::AuthFailCtxNotInRequestExt,
        })
    }
}
