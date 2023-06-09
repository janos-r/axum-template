use crate::error::*;
use axum::extract::FromRequestParts;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Ctx {
    result_user_id: Result<u64>,
    req_id: Uuid,
}

impl Ctx {
    pub fn new(result_user_id: Result<u64>, uuid: Uuid) -> Self {
        Self {
            result_user_id,
            req_id: uuid,
        }
    }

    pub fn user_id(&self) -> ApiResult<u64> {
        self.result_user_id.clone().map_err(|error| ApiError {
            error,
            req_id: self.req_id,
        })
    }

    pub fn req_id(&self) -> Uuid {
        self.req_id
    }
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
