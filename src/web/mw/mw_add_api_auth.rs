use super::error::{Error, Result};
use crate::config::{config, regenerate_replit_key};
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

    let api_configs = crate::config::config().get_api_configs();
    let first_path_segment = req.uri().path().split('/').nth(1).unwrap_or_default();
    println!("first_path_segment: {}", first_path_segment);

    let api_config = api_configs.get(first_path_segment).ok_or_else(|| {
        info!("No config found for this route");
        Error::InvalidRoute("No config found for this route".to_string())
    })?;

    let key = api_config.key.clone().unwrap();
    let auth_fn = match first_path_segment {
        "openai" => openai_auth,
        "clipdrop" => clipdrop_auth,
        "palm" => palm_auth,
        "replicate" => replicate_auth,
        "anthropic" => anthropic_auth,
        "stability" => stability_auth,
        "replit" => replit_auth,
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

pub fn openai_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_bearer_header(req, auth);
}

pub fn clipdrop_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_x_api_key_header(req, auth);
}

pub fn palm_auth<B>(req: &mut Request<B>, auth: &str) {
    add_key_query_param(req, auth);
}

pub fn replicate_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_token_header(req, auth);
}

pub fn anthropic_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_x_api_key_header(req, auth);
    req.headers_mut().insert(
        "anthropic-version",
        HeaderValue::from_str("2023-06-01").unwrap(),
    );
}

pub fn stability_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_bearer_header(req, auth);
}

pub fn replit_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_bearer_header(req, auth);
}
