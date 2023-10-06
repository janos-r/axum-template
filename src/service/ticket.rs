use crate::{
    ctx::Ctx,
    error::{ApiError, ApiResult, Error},
    Db,
};
use async_graphql::{ComplexObject, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Ticket {
    #[graphql(skip)]
    pub id: Option<Thing>,
    pub creator: String,
    pub title: String,
}
#[ComplexObject]
impl Ticket {
    async fn id(&self) -> String {
        self.id.as_ref().map(|t| &t.id).expect("id").to_raw()
    }
}

#[derive(Deserialize, InputObject)]
pub struct CreateTicketInput {
    pub title: String,
}

pub struct TicketService<'a> {
    pub db: &'a Db,
    pub ctx: &'a Ctx,
}
impl<'a> TicketService<'a> {
    pub async fn list_tickets(&self) -> ApiResult<Vec<Ticket>> {
        self.db
            .select("tickets")
            .await
            .map_err(ApiError::from(self.ctx))
    }

    pub async fn create_ticket(&self, ct_input: CreateTicketInput) -> ApiResult<Ticket> {
        self.db
            .create("tickets")
            .content(Ticket {
                id: None,
                creator: self.ctx.user_id()?,
                title: ct_input.title,
            })
            .await
            .map_err(ApiError::from(self.ctx))
            .map(|v: Vec<Ticket>| v.into_iter().next().expect("created ticket"))
    }

    pub async fn delete_ticket(&self, id: String) -> ApiResult<Ticket> {
        // NOTE: If the input is parsed from Thing format
        // let t = thing(&id).map_err(|e| ApiError {
        //     req_id: self.ctx.req_id(),
        //     error: Error::SurrealDbParse {
        //         source: e.to_string(),
        //         id: id.clone(),
        //     },
        // })?;
        self.db
            // .delete(t)
            .delete(("tickets", &id))
            .await
            .map_err(ApiError::from(self.ctx))?
            .ok_or(ApiError {
                req_id: self.ctx.req_id(),
                error: Error::SurrealDbNoResult {
                    source: "internal".to_string(),
                    id,
                },
            })
    }
}
