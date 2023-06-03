use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type ApiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// API version - this is visible in the gql doc!
    async fn version(&self) -> &str {
        "1.0"
    }
}
