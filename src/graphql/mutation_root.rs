mod tickets_mutation;
mod tickets_no_db_mutation;
use async_graphql::Object;
use tickets_mutation::TicketsMutation;
use tickets_no_db_mutation::TicketsNoDbMutation;

pub struct MutationRoot;
#[Object]
impl MutationRoot {
    async fn tickets(&self) -> TicketsMutation {
        TicketsMutation
    }

    async fn tickets_no_db(&self) -> TicketsNoDbMutation {
        TicketsNoDbMutation
    }
}
