#![allow(unused)] // For early development.

use crate::model::ModelManager;

use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod crypt;
mod ctx;
mod error;
mod lightning;
mod log;
mod model;
mod utils;
mod web;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    // dotenv().ok();

    // region:   --- Start Server
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    // _dev_utils::init_dev().await;

    // Initialize ModelManager.
    // let _mm = ModelManager::new().await?;

    let router = web::router::setup_router()?;

    // Apply middleware conditionally
    // if env::var("LNADDRESS").is_ok() && env::var("MACAROON_SECRET").is_ok() {
    //     router = router.layer(middleware::from_fn(l402::mw_l402));
    // } else {
    //     return Err(Error::RouterError {
    //         text:
    //             "No LNADDRESS or MACAROON_SECRET set, you have to set both to enable LND middleware"
    //                 .to_string(),
    //     });
    // }

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
