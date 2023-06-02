use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    Result,
};

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
            uuid: self.req_id,
        })
    }

    pub fn req_id(&self) -> Uuid {
        self.req_id
    }
}
