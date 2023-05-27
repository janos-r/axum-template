use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    // Hello routes
    async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
        let name = params.name.as_deref().unwrap_or("World");
        Html(format!("Hello <strong>{name}!!!</strong>"))
    }

    async fn handle_hello2(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
        Html(format!("Hello <strong>{name}!!!</strong>"))
    }

    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handle_hello))
            .route("/hello2/:x", get(handle_hello2))
    }

    // fallback fs
    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }
}
