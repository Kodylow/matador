use crate::web::middleware::l402;

pub use self::error::{Error, Result};

use axum::{middleware, Router};
use std::net::SocketAddr;
use tracing::info;

mod error;
mod services;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // region:   --- Start Server
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .nest("/openai", web::routes_openai::routes())
        .nest("/clipdrop", web::routes_clipdrop::routes())
        .nest("/makersuite", web::routes_makersuite::routes())
        .layer(middleware::from_fn(l402::mw_l402));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
