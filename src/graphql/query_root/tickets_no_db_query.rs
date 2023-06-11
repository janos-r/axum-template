use crate::model_no_db::{ModelController, TicketNoDb};
use async_graphql::{Context, Object, Result};

pub struct TicketsNoDbQuery;
#[Object]
impl TicketsNoDbQuery {
    async fn list(&self, ctx: &Context<'_>) -> Result<Vec<TicketNoDb>> {
        let mc = ctx.data::<ModelController>()?;
        Ok(mc.list_tickets().await)
    }
}
