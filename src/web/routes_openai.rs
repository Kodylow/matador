use std::env;

use crate::{Error, Result};
use axum::{
    http::{HeaderMap, HeaderValue},
    routing, Json, Router,
};
use serde_json::{json, Value};

use super::OPENAI_API_KEY;

pub fn routes() -> Router {
    Router::new().route("/v1/models", routing::get(list_models))
}

#[axum::debug_handler]
pub async fn list_models() -> Result<Json<Value>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", OPENAI_API_KEY.clone())).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client
        .get("https://api.openai.com/v1/models")
        .send()
        .await
        .unwrap();

    let models: Value = res.json().await.unwrap();

    Ok(Json(models))
}

// #[derive(Debug, Deserialize)]
// struct LoginPayload {
//     username: String,
//     pwd: String,
// }
