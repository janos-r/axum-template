use crate::{
    ctx::Ctx,
    model_no_db::{ModelController, TicketNoDb},
    service::ticket::CreateTicketInput,
};
use async_graphql::{Context, Object, Result};

pub struct TicketsNoDbMutation;
#[Object]
impl TicketsNoDbMutation {
    async fn create_ticket(
        &self,
        ctx: &Context<'_>,
        ct_input: CreateTicketInput,
    ) -> Result<TicketNoDb> {
        let mc = ctx.data::<ModelController>()?;
        let ctx = ctx.data::<Ctx>()?;
        Ok(mc.create_ticket(ctx, ct_input).await?)
    }

    async fn delete_ticket(&self, ctx: &Context<'_>, id: u64) -> Result<TicketNoDb> {
        let mc = ctx.data::<ModelController>()?;
        let ctx = ctx.data::<Ctx>()?;
        Ok(mc.delete_ticket(ctx, id).await?)
    }
}
