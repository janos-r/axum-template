use crate::{
    ctx::Ctx,
    service::ticket::{Ticket, TicketService},
    Db,
};
use async_graphql::{Context, Object, Result};

pub struct TicketsQuery;
#[Object]
impl TicketsQuery {
    async fn list(&self, ctx: &Context<'_>) -> Result<Vec<Ticket>> {
        let db = ctx.data::<Db>()?;
        let ctx = ctx.data::<Ctx>()?;
        Ok(TicketService { db, ctx }.list_tickets().await?)
    }
}
