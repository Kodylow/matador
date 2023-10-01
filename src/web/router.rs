// src/router.rs

use crate::config::apis::{apis_config, ApiParams, ApisConfig};
use crate::error::{Error, Result};
use axum::routing::get;
use axum::{middleware, Router};
use reverse_proxy_service::TrimPrefix;

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

        router = router.nest("/", subrouter);
    }

    router = router.layer(middleware::from_fn(add_auth));

    Ok(router)
}

fn get_params_per_api_keys_set() -> Vec<ApiParams> {
    let api_configs = apis_config();

    let mut routes = Vec::new();

    if let Some(key) = &api_configs.openai.key {
        routes.push(api_configs.openai.clone());
    }
    if let Some(key) = &api_configs.clipdrop.key {
        routes.push(api_configs.clipdrop.clone());
    }
    if let Some(key) = &api_configs.palm.key {
        routes.push(api_configs.palm.clone());
    }
    if let Some(key) = &api_configs.replicate.key {
        routes.push(api_configs.replicate.clone());
    }
    if let Some(key) = &api_configs.anthropic.key {
        routes.push(api_configs.anthropic.clone());
    }
    if let Some(key) = &api_configs.stability.key {
        routes.push(api_configs.stability.clone());
    }
    if let Some(key) = &api_configs.goose.key {
        routes.push(api_configs.goose.clone());
    }
    if let Some(key) = &api_configs.cohere.key {
        routes.push(api_configs.cohere.clone());
    }
    if let Some(key) = &api_configs.ai21.key {
        routes.push(api_configs.ai21.clone());
    }
    // if let Some(key) = &api_configs.get_replit_key() {
    //     routes.push(api_configs.replit.lock().unwrap().clone());
    // }

    routes
}
