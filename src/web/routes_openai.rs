use std::sync::Arc;

use crate::services::openai::{self, OpenAI};
use crate::{services, Error, Result};
use axum::extract::{Path, State};
use axum::{routing, Json, Router};
use serde_json::{json, Value};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    openai: Arc<Mutex<services::openai::OpenAI>>,
}

pub fn routes() -> Router {
    Router::new()
        .route(
            "/v1/models",
            routing::get(list_models).with_state(AppState {
                openai: OpenAI::get_instance(),
            }),
        )
        .route(
            "/v1/chat/completions",
            routing::post(chat_completion_create).with_state(AppState {
                openai: OpenAI::get_instance(),
            }),
        )
        .route(
            "/v1/models/:model_id",
            routing::get(retrieve_model).with_state(AppState {
                openai: OpenAI::get_instance(),
            }),
        )
}

#[axum::debug_handler]
pub async fn list_models(State(app_state): State<AppState>) -> Result<Json<Value>> {
    let openai = app_state.openai.lock().await;
    let models = openai.model_list().await?;
    Ok(Json(models))
}

#[axum::debug_handler]
pub async fn retrieve_model(
    State(app_state): State<AppState>,
    Path(model_id): Path<String>,
) -> Result<Json<Value>> {
    let openai = app_state.openai.lock().await;
    let model = openai.model_retrieve(&model_id).await?;
    Ok(Json(model))
}

#[axum::debug_handler]
pub async fn chat_completion_create(
    State(app_state): State<AppState>,
    Json(req): Json<openai::types::ChatCompletionRequest>,
) -> Result<Json<openai::types::ChatCompletionResponse>> {
    let openai = app_state.openai.lock().await;
    let response = openai.create_chat_completion(req).await?;
    println!("response: {:?}", response);
    Ok(Json(response))
}
