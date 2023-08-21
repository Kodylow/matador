pub use self::error::{Error, Result};

use axum::Router;
use std::net::SocketAddr;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // region:   --- Start Server
    let router = Router::new().nest("/openai", web::routes_openai::routes());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
