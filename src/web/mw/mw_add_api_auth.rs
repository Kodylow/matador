use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::{HeaderName, HeaderValue, Request};
use axum::middleware::Next;
use axum::response::Response;
use base64_url::base64;
use tracing::{debug, info};

use super::error::{Error, Result};
use crate::config::apis::apis_config;
use crate::config::config::config;
use crate::utils::{
    add_key_query_param, insert_auth_basic_header, insert_auth_bearer_header,
    insert_auth_token_header, insert_x_api_key_header, remove_host_header,
};

pub async fn add_auth<B: std::fmt::Debug>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_add_auth", "MIDDLEWARE");
    remove_host_header(&mut req);

    let first_path_segment = req.uri().path().split('/').nth(1).unwrap_or_default();

    info!("first_path_segment: {}", first_path_segment);
    if first_path_segment == "replit" {
        let mut replit_config = match apis_config().replit.clone() {
            Some(replit_config) => replit_config,
            None => {
                info!("No replit config found");
                return Err(Error::InvalidRoute("No replit config found".to_string()));
            }
        };

        let key = replit_config.get_key();
            
        bearer_auth(&mut req, &key);

        return Ok(next.run(req).await);
    }

    let api_config = apis_config().get_params(first_path_segment);
    let key = match api_config {
        Some(api_config) => api_config.key,
        None => {
            info!("No key found for this route");
            return Err(Error::InvalidRoute(
                "No key found for this route".to_string(),
            ));
        }
    };

    let auth_fn = match first_path_segment {
        "openai" => bearer_auth,
        "clipdrop" => x_api_key_auth,
        "palm" => key_query_param_auth,
        "replicate" => token_auth,
        "anthropic" => anthropic_auth,
        "stability" => bearer_auth,
        "goose" => bearer_auth,
        "cohere" => bearer_auth,
        "ai21" => bearer_auth,
        "together" => bearer_auth,
        "scenario" => basic_auth,
        "perplexity" => bearer_auth,
        "anyscale" => bearer_auth,
        "bing" => bing_auth,
        _ => {
            info!("No auth found for this route");
            return Err(Error::InvalidRoute(
                "No auth found for this route".to_string(),
            ));
        }
    };

    auth_fn(&mut req, &key);

    info!("URI: {:?}", req.uri());

    Ok(next.run(req).await)
}

pub fn bearer_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_bearer_header(req, auth);
}

pub fn basic_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_basic_header(req, auth);
}

pub fn x_api_key_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_x_api_key_header(req, auth);
}

pub fn key_query_param_auth<B>(req: &mut Request<B>, auth: &str) {
    add_key_query_param(req, auth);
}

pub fn token_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_token_header(req, auth);
}

pub fn anthropic_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_x_api_key_header(req, auth);
    req.headers_mut().insert(
        "anthropic-version",
        HeaderValue::from_str("2023-06-01").unwrap(),
    );
}

pub fn bing_auth<B>(req: &mut Request<B>, auth: &str) {
    req.headers_mut().insert(
        HeaderName::from_str("Ocp-Apim-Subscription-Key").unwrap(),
        HeaderValue::from_str(auth).unwrap(),
    );
}
