// src/router.rs

use axum::routing::get;
use axum::{middleware, Router};
use reverse_proxy_service::TrimPrefix;
use tracing::info;

use super::mw::mw_add_api_auth::add_auth;
use super::mw::mw_l402::mw_l402;
use crate::config::apis::{apis_config, ApiParams, ApisConfig};
use crate::error::{Error, Result};
use crate::web::routes_static;
use http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn setup_router() -> Result<Router> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let mut router = Router::new();
    let router = set_api_proxy_routes(router)?;
    let router = set_l402_wrapper(router)?;
    let router = router
        .layer(cors)
        .fallback_service(routes_static::serve_dir());

    Ok(router)
}

fn set_l402_wrapper(mut router: Router) -> Result<Router> {
    router = router.layer(middleware::from_fn(mw_l402));
    Ok(router)
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn set_api_proxy_routes(mut router: Router) -> Result<Router> {
    let params = apis_config().get_params_per_api_keys_set();

    if params.is_empty() {
        return Err(Error::RouterFailToSetRoutes(
            "No routes set, check environment variables".into(),
        ));
    }

    for p in &params {
        let host = reverse_proxy_service::builder_https(p.host).unwrap();
        let service = host.build(TrimPrefix(p.path));
        let subrouter = Router::new().nest_service(p.path, service);

        info!("Setting routing for service: {}", p.path);

        router = router.nest("/", subrouter);
    }

    router = router.layer(middleware::from_fn(add_auth));

    Ok(router)
}
