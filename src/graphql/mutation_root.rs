use crate::{
    ctx::Ctx,
    model::{CreateTicketInput, ModelController, Ticket},
};
use async_graphql::{Context, Object, Result};

pub struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn tickets(&self) -> TicketsMutation {
        TicketsMutation
    }
}

struct TicketsMutation;
#[Object]
impl TicketsMutation {
    async fn create_ticket(
        &self,
        ctx: &Context<'_>,
        ct_input: CreateTicketInput,
    ) -> Result<Ticket> {
        let mc = ctx.data::<ModelController>()?;
        let ctx = ctx.data::<Ctx>()?;
        Ok(mc.create_ticket(ctx, ct_input).await?)
    }

    async fn delete_ticket(&self, ctx: &Context<'_>, id: u64) -> Result<Ticket> {
        let mc = ctx.data::<ModelController>()?;
        let ctx = ctx.data::<Ctx>()?;
        Ok(mc.delete_ticket(ctx, id).await?)
    }
}
