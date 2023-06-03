mod ctx;
mod error;
mod graphql;
mod model;
mod mw_ctx;
mod mw_req_logger;
mod web;

use self::error::*;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    middleware,
    response::{self, IntoResponse},
    routing::{get, get_service},
    Router,
};
use graphql::model::{ApiSchema, QueryRoot};
use model::ModelController;
use mw_req_logger::mw_req_logger;
// use starwars::{QueryRoot, StarWars, StarWarsSchema};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

async fn graphql_handler(
    // schema: Extension<StarWarsSchema>,
    schema: Extension<ApiSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> ApiResult<()> {
    let mc = ModelController::new().await?;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        // .data(StarWars::new())
        .finish();

    let gql = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let routes_tickets = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));
    let routes_all = Router::new()
        .merge(gql)
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .nest("/api", routes_tickets)
        .layer(middleware::map_response(mw_req_logger))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_ctx::mw_ctx_constructor,
        ))
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
