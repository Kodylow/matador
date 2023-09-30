use super::error::{Error, Result};
use crate::config::config;
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

#[derive(Debug)]
pub enum Auth {
    OpenAI,
    ClipDrop,
    Palm,
    Replicate,
    Anthropic,
    Replit,
}

impl Auth {
    fn key(&self) -> &'static str {
        match self {
            Auth::OpenAI => config().OPENAI_API_KEY.as_ref().unwrap(),
            Auth::ClipDrop => config().CLIPDROP_API_KEY.as_ref().unwrap(),
            Auth::Palm => config().PALM_API_KEY.as_ref().unwrap(),
            Auth::Replicate => config().REPLICATE_API_KEY.as_ref().unwrap(),
            Auth::Anthropic => config().ANTHROPIC_API_KEY.as_ref().unwrap(),
            Auth::Replit => config().REPLIT_API_KEY.as_ref().unwrap(),
        }
    }

    fn auth_fn<B>(&self) -> fn(&mut Request<B>, &str) {
        match self {
            Auth::OpenAI => openai_auth,
            Auth::ClipDrop => clipdrop_auth,
            Auth::Palm => palm_auth,
            Auth::Replicate => replicate_auth,
            Auth::Anthropic => anthropic_auth,
            Auth::Replit => replit_auth,
        }
    }
}

pub async fn add_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    debug!("{:<12} - mw_add_auth", "MIDDLEWARE");
    remove_host_header(&mut req);

    let first_path_segment = req.uri().path().split('/').nth(1).unwrap_or_default();

    println!("first_path_segment: {}", first_path_segment);
    let auth = match first_path_segment {
        "openai" => Auth::OpenAI,
        "clipdrop" => Auth::ClipDrop,
        "palm" => Auth::Palm,
        "replicate" => Auth::Replicate,
        "anthropic" => Auth::Anthropic,
        "replit" => Auth::Replit,
        _ => {
            info!("No auth found for this route");
            return Err(Error::InvalidRoute(
                "No auth found for this route".to_string(),
            ));
        }
    };

    auth.auth_fn()(&mut req, auth.key());

    info!("Headers: {:?}", req.headers());
    info!("URI: {:?}", req.uri());

    Ok(next.run(req).await)
}

fn openai_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_bearer_header(req, auth);
}

fn clipdrop_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_x_api_key_header(req, auth);
}

fn palm_auth<B>(req: &mut Request<B>, auth: &str) {
    add_key_query_param(req, auth);
}

fn replicate_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_token_header(req, auth);
}

fn anthropic_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_x_api_key_header(req, auth);
    req.headers_mut().insert(
        "anthropic-version",
        HeaderValue::from_str("2023-06-01").unwrap(),
    );
}

fn replit_auth<B>(req: &mut Request<B>, auth: &str) {
    insert_auth_bearer_header(req, auth);
}