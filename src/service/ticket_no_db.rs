use crate::{
    ctx::Ctx,
    error::{ApiError, Error, Result},
    service::ticket::CreateTicketInput,
    ApiResult,
};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct TicketNoDb {
    pub id: u64,
    pub creator: String,
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<TicketNoDb>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    pub async fn create_ticket(
        &self,
        ctx: &Ctx,
        ct_input: CreateTicketInput,
    ) -> ApiResult<TicketNoDb> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = TicketNoDb {
            id,
            creator: ctx.user_id()?,
            title: ct_input.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Vec<TicketNoDb> {
        let store = self.tickets_store.lock().unwrap();
        store.iter().flatten().cloned().collect()
    }

    pub async fn delete_ticket(&self, ctx: &Ctx, id: u64) -> ApiResult<TicketNoDb> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(Option::take);
        ticket.ok_or(ApiError {
            req_id: ctx.req_id(),
            error: Error::TicketDeleteFailIdNotFound { id },
        })
    }
}
