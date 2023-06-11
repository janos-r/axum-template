use crate::ctx::Ctx;
use crate::service::ticket::{CreateTicketInput, Ticket, TicketService};
use crate::{ApiResult, Db};
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(db)
}

async fn list_tickets(State(db): State<Db>, ctx: Ctx) -> ApiResult<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    TicketService { db: &db, ctx: &ctx }
        .list_tickets()
        .await
        .map(Json)
}

async fn create_ticket(
    State(db): State<Db>,
    ctx: Ctx,
    Json(ct_input): Json<CreateTicketInput>,
) -> ApiResult<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    TicketService { db: &db, ctx: &ctx }
        .create_ticket(ct_input)
        .await
        .map(Json)
}

async fn delete_ticket(
    State(db): State<Db>,
    ctx: Ctx,
    Path(id): Path<String>,
) -> ApiResult<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    TicketService { db: &db, ctx: &ctx }
        .delete_ticket(id)
        .await
        .map(Json)
}
