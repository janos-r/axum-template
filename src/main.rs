#![allow(unused)]

mod error;
mod model;
mod web;

use self::error::{Error, Result};
use axum::response::Response;
use axum::routing::get_service;
use axum::{middleware, Router};
use model::ModelController;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    let routes_all = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    async fn main_response_mapper(res: Response) -> Response {
        println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
        println!();
        res
    }

    // fallback fs
    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }

    Ok(())
}
