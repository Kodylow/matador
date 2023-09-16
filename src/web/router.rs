// src/router.rs

use crate::error::{Error, Result};
use axum::{middleware, Router};
use reverse_proxy_service::TrimPrefix;

use super::mw::mw_add_api_auth::{
    add_clipdrop_auth, add_openai_auth, add_palm_auth, add_replicate_auth,
};
use super::mw::mw_l402::mw_l402;

#[derive(Clone)]
enum Route {
    OpenAI,
    ClipDrop,
    Palm,
    Replicate,
}

impl Route {
    fn host(&self) -> &'static str {
        match self {
            Route::OpenAI => "api.openai.com",
            Route::ClipDrop => "clipdrop-api.co",
            Route::Palm => "generativelanguage.googleapis.com",
            Route::Replicate => "api.replicate.com",
        }
    }

    fn path(&self) -> &'static str {
        match self {
            Route::OpenAI => "/openai",
            Route::ClipDrop => "/clipdrop",
            Route::Palm => "/palm",
            Route::Replicate => "/replicate",
        }
    }
}

pub fn setup_router() -> Result<Router> {
    let router = Router::new();
    let router = set_api_proxy_routes(router)?;
    let router = set_l402_wrapper(router)?;

    Ok(router)
}

fn set_api_proxy_routes(mut router: Router) -> Result<Router> {
    let routes = [
        Route::OpenAI,
        Route::ClipDrop,
        Route::Palm,
        Route::Replicate,
    ];

    let mut routes_set = false;

    for route in &routes {
        let host = reverse_proxy_service::builder_https(route.host()).unwrap();
        let service = host.build(TrimPrefix(route.path()));
        let mut subrouter = Router::new().nest_service(route.path(), service);
        match route {
            Route::OpenAI => {
                subrouter = subrouter.layer(middleware::from_fn(add_openai_auth));
                routes_set = true;
            }
            Route::ClipDrop => {
                subrouter = subrouter.layer(middleware::from_fn(add_clipdrop_auth));
                routes_set = true;
            }
            Route::Palm => {
                subrouter = subrouter.layer(middleware::from_fn(add_palm_auth));
                routes_set = true;
            }
            Route::Replicate => {
                subrouter = subrouter.layer(middleware::from_fn(add_replicate_auth));
                routes_set = true;
            }
        }

        router = router.nest("/", subrouter);
    }

    if !routes_set {
        return Err(Error::RouterFailToSetRoutes(
            "No routes set, check environment variables".into(),
        ));
    }

    Ok(router)
}

fn set_l402_wrapper(mut router: Router) -> Result<Router> {
    router = router.layer(middleware::from_fn(mw_l402));

    Ok(router)
}
