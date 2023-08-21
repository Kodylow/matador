pub use self::error::{Error, Result};

use axum::Router;
use std::net::SocketAddr;
use tracing::info;

mod error;
mod services;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // region:   --- Start Server
    let router = Router::new().nest("/openai", web::routes_openai::routes());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
