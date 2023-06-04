use crate::model::{ModelController, Ticket};
use async_graphql::{Context, Object, Result};

pub struct QueryRoot;
#[Object]
impl QueryRoot {
    /// API version - this is visible in the gql doc!
    async fn version(&self) -> &str {
        "1.0"
    }

    async fn tickets(&self) -> TicketsQuery {
        TicketsQuery
    }
}

struct TicketsQuery;
#[Object]
impl TicketsQuery {
    async fn list(&self, ctx: &Context<'_>) -> Result<Vec<Ticket>> {
        let mc = ctx.data::<ModelController>()?;
        Ok(mc.list_tickets().await)
    }
}
