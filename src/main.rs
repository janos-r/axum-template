mod ctx;
mod error;
mod graphql;
mod mw_ctx;
mod mw_req_logger;
mod service;
mod web;

use async_graphql::{EmptySubscription, Schema};
use axum::{
    extract::Extension,
    middleware,
    routing::{get, get_service},
    Router,
};
use error::{ApiResult, Result};
use graphql::{
    graphiql, graphql_handler, mutation_root::MutationRoot, query_root::QueryRoot, ApiSchema,
};
use mw_req_logger::mw_req_logger;
use service::ticket_no_db::ModelController;
use std::net::{Ipv4Addr, SocketAddr};
use surrealdb::{
    engine::local::{Db as LocalDb, Mem},
    Surreal,
};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

type Db = surrealdb::Surreal<LocalDb>;
static DB: Db = Surreal::init();

#[tokio::main]
async fn main() -> Result<()> {
    // no-DB in-memory
    let mc = ModelController::new().await?;

    // DB
    // NOTE: For connection to an existing DB
    // let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8000));
    // let db = Surreal::new::<Ws>(addr).await?;
    // NOTE: Also possible to start the DB with ::new without a static ::init
    // let db: Db = Surreal::new::<Mem>(()).await?;
    DB.connect::<Mem>(()).await?;
    println!("->> DB connected in memory");
    let version = DB.version().await?;
    println!("->> DB version: {version}");
    // Select a specific namespace / database
    DB.use_ns("namespace").use_db("database").await?;

    // GQL
    let schema: ApiSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mc.clone())
        .data(DB.clone())
        .finish();
    let gql = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        // Require auth to access gql
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));

    // REST
    let routes_tickets_no_db = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));

    let routes_all = Router::new()
        // No requirements
        .merge(web::routes_hello::routes())
        // Also behind /api, but no auth requirement on this route
        .merge(web::routes_login::routes())
        .merge(gql)
        .nest("/noDb", routes_tickets_no_db)
        .layer(middleware::map_response(mw_req_logger))
        // This is where Ctx gets created, with every new request
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_ctx::mw_ctx_constructor,
        ))
        // Layers are executed from bottom up, so CookieManager has to me under ctx_constructor
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8080));
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
