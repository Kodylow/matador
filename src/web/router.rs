// src/router.rs

use crate::config::apis::{apis_config, ApiParams, ApisConfig};
use crate::error::{Error, Result};
use axum::routing::get;
use axum::{middleware, Router};
use reverse_proxy_service::TrimPrefix;
use tracing::info;

use super::mw::mw_add_api_auth::add_auth;
use super::mw::mw_l402::mw_l402;

pub fn setup_router() -> Result<Router> {
    let router = Router::new();
    let router = set_api_proxy_routes(router)?;
    let router = set_l402_wrapper(router)?;

    Ok(router)
}

fn set_l402_wrapper(mut router: Router) -> Result<Router> {
    router = router
        .route("/", get(root))
        .layer(middleware::from_fn(mw_l402));

    info!("Setting l402 wrapper");
    Ok(router)
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn set_api_proxy_routes(mut router: Router) -> Result<Router> {
    let params = get_params_per_api_keys_set();

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

fn get_params_per_api_keys_set() -> Vec<ApiParams> {
    let api_configs = apis_config();
    let api_params = [
        &api_configs.openai,
        &api_configs.clipdrop,
        &api_configs.palm,
        &api_configs.replicate,
        &api_configs.anthropic,
        &api_configs.stability,
        &api_configs.goose,
        &api_configs.cohere,
        &api_configs.ai21,
        &api_configs.together,
        &api_configs.scenario,
        &api_configs.replit.clone().map(|replit| replit.params),
    ];

    api_params
        .iter()
        .filter_map(|api_param| api_param.as_ref().cloned())
        .collect()
}
