use super::error::{Error, Result};
use crate::config::config;
use axum::http;
use axum::{
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use tracing::debug;

const HOST: &str = "host";
const X_API_KEY: &str = "x-api-key";
const AUTHORIZATION: &str = "authorization";

fn remove_host_header<B>(req: &mut Request<B>) {
    req.headers_mut().remove(HOST);
}

pub async fn add_openai_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_openai", "MIDDLEWARE");
    remove_host_header(&mut req);

    let auth = HeaderValue::from_str(&format!(
        "Bearer {}",
        config().OPENAI_API_KEY.as_ref().unwrap()
    ))?;

    req.headers_mut().insert(AUTHORIZATION, auth);

    Ok(next.run(req).await)
}

pub async fn add_clipdrop_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_clipdrop", "MIDDLEWARE");
    remove_host_header(&mut req);

    let auth = HeaderValue::from_str(&format!("{}", config().CLIPDROP_API_KEY.as_ref().unwrap()))?;
    req.headers_mut().insert(X_API_KEY, auth);

    Ok(next.run(req).await)
}

pub async fn add_makersuite_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_makersuite", "MIDDLEWARE");
    remove_host_header(&mut req);

    // add key to query params like ?key=$MAKERSUITE_API_KEY
    let mut parts = req.uri().clone().into_parts();
    let key_param = format!("key={}", config().MAKERSUITE_API_KEY.as_ref().unwrap());
    parts.path_and_query = Some(
        http::uri::PathAndQuery::from_maybe_shared(format!(
            "{}?{}",
            parts
                .path_and_query
                .map(|pq| pq.path().to_string())
                .unwrap_or_default(),
            key_param
        ))
        .unwrap(),
    );
    *req.uri_mut() = http::Uri::from_parts(parts).unwrap();

    Ok(next.run(req).await)
}

pub async fn add_replicate_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_replicate", "MIDDLEWARE");
    remove_host_header(&mut req);

    let auth = HeaderValue::from_str(&format!(
        "Token {}",
        config().REPLICATE_API_KEY.as_ref().unwrap()
    ))?;

    req.headers_mut().insert(AUTHORIZATION, auth);

    Ok(next.run(req).await)
}
