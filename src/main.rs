use crate::web::middleware::l402;

pub use self::error::{Error, Result};

use axum::{middleware, Router};
use std::net::SocketAddr;
use tracing::info;

mod error;
mod services;
mod web;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // region:   --- Start Server
    tracing_subscriber::fmt::init();
    let mut router = Router::new();
    let mut routes_set = false;

    // Add routes conditionally based on environment variables
    if env::var("OPENAI").is_ok() {
        router = router.nest("/openai", web::routes_openai::routes());
        routes_set = true;
    }
    if env::var("CLIPDROP").is_ok() {
        router = router.nest("/clipdrop", web::routes_clipdrop::routes());
        routes_set = true;
    }
    if env::var("MAKERSUITE").is_ok() {
        router = router.nest("/makersuite", web::routes_makersuite::routes());
        routes_set = true;
    }

    // Apply middleware conditionally
    if env::var("LNADDRESS").is_ok() && env::var("MACAROON_SECRET").is_ok() {
        router = router.layer(middleware::from_fn(l402::mw_l402));
    } else {
        return Err(Error::new(
            "Middleware cannot be set because LNADDRESS and/or MACAROON_SECRET are not set",
        ));
    }

    // Check if any routes are set
    if !routes_set {
        return Err(Error::new("No routes are set"));
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
