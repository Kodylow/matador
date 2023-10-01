use super::error::{Error, Result};
use crate::config::apis::apis_config;
use crate::config::config::config;
use crate::utils::{
    add_key_query_param, insert_auth_bearer_header, insert_auth_token_header,
    insert_x_api_key_header, remove_host_header,
};

use axum::{
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use tracing::{debug, info};

pub async fn add_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_add_auth", "MIDDLEWARE");
    remove_host_header(&mut req);

    let first_path_segment = req.uri().path().split('/').nth(1).unwrap_or_default();
    println!("first_path_segment: {}", first_path_segment);

    let api_config = apis_config().get_params(first_path_segment);
    let key = match api_config {
        Some(a) => a.key.clone().unwrap(),
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
        // "replit" => bearer_auth,
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
