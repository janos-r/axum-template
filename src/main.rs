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
use jsonwebtoken::{DecodingKey, EncodingKey};
use mw_ctx::CtxState;
use mw_req_logger::mw_req_logger;
use once_cell::sync::Lazy;
use service::ticket_no_db::ModelController;
use std::net::{Ipv4Addr, SocketAddr};
use surrealdb::{
    engine::local::{Db as LocalDb, Mem},
    Surreal,
};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

type Db = Surreal<LocalDb>;
static DB: Lazy<Db> = Lazy::new(Surreal::init);

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
    let routes_tickets_no_db = web::routes_tickets_no_db::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));
    let routes_tickets = web::routes_tickets::routes(DB.clone())
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));

    // Load secret and create secret key for JWT
    let secret = "some-secret".as_bytes();
    let key_enc = EncodingKey::from_secret(secret);
    let key_dec = DecodingKey::from_secret(secret);
    let ctx_state = CtxState {
        _db: DB.clone(),
        key_enc,
        key_dec,
    };

    // Main router
    let routes_all = Router::new()
        // No requirements
        .merge(web::routes_hello::routes())
        // Also behind /api, but no auth requirement on this route
        .merge(web::routes_login::routes(ctx_state.clone()))
        .merge(gql)
        .nest("/noDb", routes_tickets_no_db)
        .nest("/api", routes_tickets)
        .layer(middleware::map_response(mw_req_logger))
        // This is where Ctx gets created, with every new request
        .layer(middleware::from_fn_with_state(
            ctx_state.clone(),
            mw_ctx::mw_ctx_constructor,
        ))
        // Layers are executed from bottom up, so CookieManager has to be under ctx_constructor
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("->> LISTENING on {addr}\n");
    axum::serve(listener, routes_all).await.unwrap();

    // fallback fs
    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }

    Ok(())
}
