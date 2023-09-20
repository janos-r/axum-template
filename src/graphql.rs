pub mod mutation_root;
pub mod query_root;

use crate::{ctx::Ctx, error::Error, error::ERROR_SER_KEY};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, Value};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
};
use mutation_root::MutationRoot;
use query_root::QueryRoot;

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn graphql_handler(
    schema: Extension<ApiSchema>,
    ctx: Ctx,
    req: async_graphql_axum::GraphQLRequest,
) -> axum::response::Response {
    let mut gql_resp: async_graphql::Response = schema.execute(req.into_inner().data(ctx)).await;

    // Remove error extensions and deserialize errors
    let mut error: Option<Error> = None;
    for gql_error in &mut gql_resp.errors {
        let Some(extensions) = &mut gql_error.extensions else {
            continue;
        };
        let Some(value) = extensions.get(ERROR_SER_KEY) else {
            continue;
        };
        let Value::String(s) = value else { continue };
        error = Some(serde_json::from_str(s).unwrap_or_else(Error::from));
        extensions.unset(ERROR_SER_KEY);
        break;
    }
    let mut response = async_graphql_axum::GraphQLResponse::from(gql_resp).into_response();
    // Insert the real Error into the response - for the logger
    if let Some(e) = error {
        response.extensions_mut().insert(e);
    }
    response
}
