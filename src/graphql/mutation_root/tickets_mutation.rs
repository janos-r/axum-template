use crate::{
    ctx::Ctx,
    service::ticket::{CreateTicketInput, Ticket, TicketService},
    Db,
};
use async_graphql::{Context, Object, Result};

pub struct TicketsMutation;
#[Object]
impl TicketsMutation {
    async fn create_ticket(
        &self,
        ctx: &Context<'_>,
        ct_input: CreateTicketInput,
    ) -> Result<Ticket> {
        let db = ctx.data::<Db>()?;
        let ctx = ctx.data::<Ctx>()?;
        Ok(TicketService { db, ctx }.create_ticket(ct_input).await?)
    }

    async fn delete_ticket(&self, ctx: &Context<'_>, id: String) -> Result<Ticket> {
        let db = ctx.data::<Db>()?;
        let ctx = ctx.data::<Ctx>()?;
        Ok(TicketService { db, ctx }.delete_ticket(id).await?)
    }
}
