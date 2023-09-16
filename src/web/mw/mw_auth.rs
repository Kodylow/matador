use crate::config::config;
use crate::error::{Error, Result};
use axum::{
    http::{header::AUTHORIZATION, HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use tracing::debug;

pub async fn add_openai_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_openai", "MIDDLEWARE");

    let openai_api_key = config().OPENAI_API_KEY.clone().unwrap();

    let mut req = req;

    let auth = HeaderValue::from_str(&format!("Bearer {}", openai_api_key))
        .map_err(|_| Error::ConfigMissingEnv("Config OPENAI_API_KEY is missing."))?;

    // wipe host header
    req.headers_mut().remove("host");

    // insert auth header
    req.headers_mut().insert(AUTHORIZATION, auth);

    Ok(next.run(req).await)
}

pub async fn add_clipdrop_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_clipdrop", "MIDDLEWARE");

    let clipdrop_api_key = config().CLIPDROP_API_KEY.clone().unwrap();

    let mut req = req;

    let auth = HeaderValue::from_str(&format!("{}", clipdrop_api_key))
        .map_err(|_| Error::ConfigMissingEnv("Config CLIPDROP_API_KEY is missing."))?;

    // wipe host header
    req.headers_mut().remove("host");

    // insert auth header
    req.headers_mut().insert("x-api-key", auth);

    Ok(next.run(req).await)
}
