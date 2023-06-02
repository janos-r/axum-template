mod ctx;
mod error;
mod model;
mod mw_ctx;
mod mw_req_logger;
mod web;

use self::error::*;
use axum::routing::get_service;
use axum::{middleware, Router};
use model::ModelController;
use mw_req_logger::mw_req_logger;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> ApiResult<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth));
    let routes_all = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
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
