use crate::{ctx::Ctx, ApiError, ApiResult, Error};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub creator_id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
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

    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> ApiResult<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            creator_id: ctx.user_id()?,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> ApiResult<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        let tickets: Vec<Ticket> = store.iter().flatten().cloned().collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64, ctx: Ctx) -> ApiResult<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(Option::take);
        ticket.ok_or(ApiError {
            req_id: ctx.req_id(),
            error: Error::TicketDeleteFailIdNotFound { id },
        })
    }
}
