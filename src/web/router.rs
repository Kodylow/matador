// src/router.rs

use crate::error::{Error, Result};
use axum::{middleware, Router};
use reverse_proxy_service::TrimPrefix;

use super::mw::mw_add_api_auth::add_auth;
use super::mw::mw_l402::mw_l402;

pub fn setup_router() -> Result<Router> {
    let router = Router::new();
    let router = set_api_proxy_routes(router)?;
    // let router = set_l402_wrapper(router)?;

    Ok(router)
}

fn set_l402_wrapper(mut router: Router) -> Result<Router> {
    router = router.layer(middleware::from_fn(mw_l402));
    Ok(router)
}

/// This is where you add routes for each API key set in config
/// If no API keys are set, this will return an error
/// To add a new API:
/// 1. Add a new Route enum variant with the host and path
/// 2. Add the new Route to the routes vector
/// 3. Add a new match arm to the add_auth middleware (mw_add_api_auth.rs, you have to add an auth function for the new API)
/// 4. Add a new match arm to the set_api_proxy_routes function (this file)

#[derive(Clone)]
enum Route {
    OpenAI,
    ClipDrop,
    Palm,
    Replicate,
    Anthropic,
}

impl Route {
    fn host(&self) -> &'static str {
        match self {
            Route::OpenAI => "api.openai.com",
            Route::ClipDrop => "clipdrop-api.co",
            Route::Palm => "generativelanguage.googleapis.com",
            Route::Replicate => "api.replicate.com",
            Route::Anthropic => "api.anthropic.com",
        }
    }

    fn path(&self) -> &'static str {
        match self {
            Route::OpenAI => "/openai",
            Route::ClipDrop => "/clipdrop",
            Route::Palm => "/palm",
            Route::Replicate => "/replicate",
            Route::Anthropic => "/anthropic",
        }
    }
}

fn set_api_proxy_routes(mut router: Router) -> Result<Router> {
    let routes = get_routes_per_api_keys_set();

    if routes.is_empty() {
        return Err(Error::RouterFailToSetRoutes(
            "No routes set, check environment variables".into(),
        ));
    }

    for route in &routes {
        let host = reverse_proxy_service::builder_https(route.host()).unwrap();
        let service = host.build(TrimPrefix(route.path()));
        let subrouter = Router::new().nest_service(route.path(), service);

        router = router.nest("/", subrouter);
    }

    router = router.layer(middleware::from_fn(add_auth));

    Ok(router)
}

fn get_routes_per_api_keys_set() -> Vec<Route> {
    // Check if api keys are set in config
    let mut routes: Vec<Route> = vec![];

    if crate::config::config().OPENAI_API_KEY.is_some() {
        routes.push(Route::OpenAI);
    }
    if crate::config::config().PALM_API_KEY.is_some() {
        routes.push(Route::Palm);
    }
    if crate::config::config().CLIPDROP_API_KEY.is_some() {
        routes.push(Route::ClipDrop);
    }
    if crate::config::config().REPLICATE_API_KEY.is_some() {
        routes.push(Route::Replicate);
    }
    if crate::config::config().ANTHROPIC_API_KEY.is_some() {
        routes.push(Route::Anthropic);
    }

    routes
}
