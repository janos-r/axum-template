mod ctx;
mod error;
mod graphql;
mod model;
mod mw_ctx;
mod mw_req_logger;
mod web;

use self::error::*;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, Value};
use axum::{
    extract::Extension,
    middleware,
    response::{self, IntoResponse},
    routing::{get, get_service},
    Router,
};
use ctx::Ctx;
use graphql::{mutation_root::MutationRoot, query_root::QueryRoot};
use model::ModelController;
use mw_req_logger::mw_req_logger;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
async fn graphql_handler(
    schema: Extension<ApiSchema>,
    ctx: Ctx,
    req: async_graphql_axum::GraphQLRequest,
) -> axum::response::Response {
    let mut gql_resp: async_graphql::Response = schema.execute(req.into_inner().data(ctx)).await;
    // Remove error extensions and deserialize errors
    let mut errors = Vec::new();
    for error in &mut gql_resp.errors {
        let Some(extensions) = &mut error.extensions else { continue };
        let Some(value) = extensions.get(ERROR_SER_KEY) else { continue };
        let Value::String(s) = value else { continue };
        let error: Error = serde_json::from_str(s).unwrap_or_else(Error::from);
        errors.push(error);
        extensions.unset(ERROR_SER_KEY);
    }
    // Normally only this is recommended as the result of the handler, but it seams ok to repeatedly call .into_response
    let mut response = async_graphql_axum::GraphQLResponse::from(gql_resp).into_response();
    // Insert the first real Error into the response - for the logger
    if let Some(e) = errors.into_iter().next() {
        response.extensions_mut().insert(e);
    }
    response
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> ApiResult<()> {
    // DB
    let mc = ModelController::new().await?;

    // GQL
    let schema: ApiSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mc.clone())
        .finish();
    let gql = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        // Require auth to access gql
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));

    // REST
    let routes_tickets = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));

    let routes_all = Router::new()
        // No requirements
        .merge(web::routes_hello::routes())
        // Also behind /api, but no auth requirement on this route
        .merge(web::routes_login::routes())
        .merge(gql)
        .nest("/api", routes_tickets)
        .layer(middleware::map_response(mw_req_logger))
        // This is where Ctx gets created, with every new request
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_ctx::mw_ctx_constructor,
        ))
        // Layers are executed from bottom up, so CookieManager has to me under ctx_constructor
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    // fallback fs
    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }

    Ok(())
}
