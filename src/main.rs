use crate::model::ModelManager;
use crate::web::mw::mw_auth;

use axum::{middleware, Router};
use reverse_proxy_service::TrimPrefix;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;

mod _dev_utils;

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

    // Setup Config
    let config = config::config();

    // Initialize ModelManager.
    let _mm = ModelManager::new().await?;

    let mut router = Router::new();
    let mut routes_set = false;

    // Add routes conditionally based on Config AI_API_KEYS
    if config.OPENAI_API_KEY.is_some() {
        let openai_host = reverse_proxy_service::builder_https("api.openai.com").unwrap();
        router = router
            .route_service("/openai/*path", openai_host.build(TrimPrefix("/openai")))
            .layer(middleware::from_fn(mw_auth::add_openai_auth));
        routes_set = true;
    }
    if config.CLIPDROP_API_KEY.is_some() {
        let clipdrop_host = reverse_proxy_service::builder_https("clipdrop-api.co").unwrap();
        router = router
            .route_service(
                "/clipdrop/*path",
                clipdrop_host.build(TrimPrefix("/clipdrop")),
            )
            .layer(middleware::from_fn(mw_auth::add_clipdrop_auth));
        routes_set = true;
    }
    if config.MAKERSUITE_API_KEY.is_some() {
        let makersuite_host =
            reverse_proxy_service::builder_https("generativelanguage.googleapis.com").unwrap();
        router = router
            .route_service(
                "/makersuite/*path",
                makersuite_host.build(TrimPrefix("/makersuite")),
            )
            .layer(middleware::from_fn(mw_auth::add_makersuite_auth));
        routes_set = true;
    }
    if config.REPLICATE_API_KEY.is_some() {
        let replicate_host = reverse_proxy_service::builder_https("api.replicate.com").unwrap();
        router = router
            .route_service(
                "/replicate/*path",
                replicate_host.build(TrimPrefix("/replicate")),
            )
            .layer(middleware::from_fn(mw_auth::add_replicate_auth));
        routes_set = true;
    }

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

    // Check if any routes are set

    if !routes_set {
        Err(Error::RouterFailToSetRoutes(
            "No routes set, check environment variables",
        ))?;
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
