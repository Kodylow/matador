use crate::model::ModelManager;
use crate::web::mw::mw_auth;

use axum::body::Body;
use axum::http::Request;
use axum::routing::get;
use axum::{middleware, Router};
use reverse_proxy_service::{AppendSuffix, Static, TrimPrefix};
use reverse_proxy_service::{ReplaceAll, ReusedServiceBuilder};
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
    let mm = ModelManager::new().await?;

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
    // if env::var("CLIPDROP_API_KEY").is_ok() {
    //     router = router.nest("/clipdrop", web::routes::clipdrop::routes());
    //     routes_set = true;
    // }
    // if env::var("MAKERSUITE_API_KEY").is_ok() {
    //     router = router.nest("/makersuite", web::routes::makersuite::routes());
    //     routes_set = true;
    // }

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

async fn log_test(req: Request<Body>) -> &'static str {
    info!("----- Logging request from reverse proxy");
    info!("Request: {:?}", req);
    info!("Request Headers: {:?}", req.headers());
    info!("Request Body: {:?}", req.body());
    info!("----- End Logging request from reverse proxy");

    "Hello, world!"
}
