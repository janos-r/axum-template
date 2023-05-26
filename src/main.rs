use std::net::SocketAddr;

use axum::response::IntoResponse;
use axum::routing::get;
use axum::{response::Html, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handle_hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    async fn handle_hello() -> impl IntoResponse {
        println!("->> {:<12} - handler_hello", "HANDLER");
        Html("Hello <strong>World!!!</strong>")
    }
}
