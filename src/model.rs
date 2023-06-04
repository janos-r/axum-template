use crate::{ctx::Ctx, ApiError, ApiResult, Error};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, SimpleObject)]
pub struct Ticket {
    pub id: u64,
    pub creator_id: u64,
    pub title: String,
}

#[derive(Deserialize, InputObject)]
pub struct CreateTicketInput {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> ApiResult<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    pub async fn create_ticket(&self, ctx: &Ctx, ct_input: CreateTicketInput) -> ApiResult<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            creator_id: ctx.user_id()?,
            title: ct_input.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Vec<Ticket> {
        let store = self.tickets_store.lock().unwrap();
        store.iter().flatten().cloned().collect()
    }

    pub async fn delete_ticket(&self, ctx: &Ctx, id: u64) -> ApiResult<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(Option::take);
        ticket.ok_or(ApiError {
            req_id: ctx.req_id(),
            error: Error::TicketDeleteFailIdNotFound { id },
        })
    }
}
