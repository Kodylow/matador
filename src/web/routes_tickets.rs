use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::FromRef;
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, post},
    Router,
};

#[derive(Clone, FromRef)]
// Axum has a very nice substate abstraction
// FromRef makes every attribute of the struct available as a substate which you can inject
// all your middleware and handlers can access the substate without knowing about the parent state
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(app_state)
}

// region: -- REST Handlers

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:12} - create_ticket - {ticket_fc:?}", "HANDLER");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:12} - delete_ticket - {id:?}", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}

// endregion: -- REST Handlers
