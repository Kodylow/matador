use crate::services::openai::{
    self, OpenAI, CHAT_COMPLETIONS, EMBEDDINGS, IMAGE_GENERATIONS, MODEL_LIST, MODEL_RETRIEVE,
};
use crate::{services, Result};
use axum::extract::{DefaultBodyLimit, Path};
use axum::Json;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use serde_json::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, trace};

#[derive(Clone)]
pub struct AppState {
    client: OpenAI,
}

impl AppState {
    fn new() -> Self {
        info!("Creating new AppState");
        AppState {
            client: OpenAI::new(),
        }
    }
}

pub fn routes() -> Router {
    info!("Setting up routes");
    let app_state = Arc::new(AppState::new());
    Router::new()
        .route(MODEL_LIST, get(list_models))
        .route(CHAT_COMPLETIONS, post(chat_completion_create))
        .route(MODEL_RETRIEVE, get(retrieve_model))
        .route(IMAGE_GENERATIONS, post(image_create))
        // .route(IMAGE_EDITS, post(image_edit))
        // .route(IMAGE_VARIATIONS, post(image_variation))
        .route(EMBEDDINGS, post(embeddings_create))
        // .route(TRANSCRIPTIONS, post(transcription_create))
        // .route(TRANSLATIONS, post(translation_create))
        // .route(FILES, get(files_list).post(file_upload))
        // .route(FILE, get(file_retrieve).delete(file_delete))
        // .route(FILE_CONTENT, get(file_retrieve_content))
        // Add middleware that inserts the state into all incoming request's
        // extensions.
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .layer(Extension(app_state))
}

#[axum::debug_handler]
pub async fn list_models(app_state: Extension<Arc<AppState>>) -> Result<Json<Value>> {
    info!("Calling route: list_models");
    let models = app_state.client.model_list().await?;
    Ok(Json(models))
}

#[axum::debug_handler]
pub async fn retrieve_model(
    app_state: Extension<Arc<AppState>>,
    Path(model_id): Path<String>,
) -> Result<Json<Value>> {
    info!("Calling route: retrieve_model {}", model_id);
    let model = app_state.client.model_retrieve(&model_id).await?;
    Ok(Json(model))
}

#[axum::debug_handler]
pub async fn chat_completion_create(
    app_state: Extension<Arc<AppState>>,
    Json(req): Json<openai::types::ChatCompletionRequest>,
) -> Result<Json<openai::types::ChatCompletionResponse>> {
    info!("Calling route: chat_completion_create");
    let response = app_state.client.create_chat_completion(req).await?;
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn embeddings_create(
    app_state: Extension<Arc<AppState>>,
    Json(req): Json<openai::types::EmbeddingRequest>,
) -> Result<Json<openai::types::EmbeddingResponse>> {
    info!("Calling route: embeddings");
    let response = app_state.client.create_embeddings(req).await?;
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn image_create(
    app_state: Extension<Arc<AppState>>,
    Json(req): Json<openai::types::ImageCreationRequest>,
) -> Result<Json<openai::types::ImageResponse>> {
    info!("Calling route: image_create");
    let response = app_state.client.create_image(req).await?;
    Ok(Json(response))
}

// #[axum::debug_handler]
// pub async fn image_edit(
//     app_state: Extension<Arc<AppState>>,
//     Json(req): Json<openai::types::ImageEditRequest>,
// ) -> Result<Json<openai::types::ImageResponse>> {
//     info!("Calling route: image_edit");
//     let response = app_state.client.edit_image(req).await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn image_variation(
//     app_state: Extension<Arc<AppState>>,
//     Json(req): Json<openai::types::ImageVariationRequest>,
// ) -> Result<Json<openai::types::ImageResponse>> {
//     info!("Calling route: image_vary");
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.vary_image(req).await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn transcription_create(
//     app_state: Extension<Arc<AppState>>,
//     Json(req): Json<openai::types::AudioRequest>,
// ) -> Result<Json<openai::types::AudioResponse>> {
//     info!("Calling route: transcription_create");
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.create_transcription(req).await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn translation_create(
//     app_state: Extension<Arc<AppState>>,
//     Json(req): Json<openai::types::AudioRequest>,
// ) -> Result<Json<openai::types::AudioResponse>> {
//     info!("Calling route: translation_create");
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.create_translation(req).await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn files_list(
//     app_state: Extension<Arc<AppState>>,
// ) -> Result<Json<openai::types::FileListResponse>> {
//     info!("Calling route: files_list");
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.list_files().await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn file_upload(
//     app_state: Extension<Arc<AppState>>,
//     Json(req): Json<openai::types::FileUploadRequest>,
// ) -> Result<Json<openai::types::FileResponse>> {
//     info!("Calling route: file_upload");
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.upload_file(req).await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn file_delete(
//     app_state: Extension<Arc<AppState>>,
//     Path(file_id): Path<String>,
// ) -> Result<Json<openai::types::FileDeletionResponse>> {
//     info!("Calling route: file_delete {}", file_id);
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.delete_file(&file_id).await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn file_retrieve(
//     app_state: Extension<Arc<AppState>>,
//     Path(file_id): Path<String>,
// ) -> Result<Json<openai::types::FileResponse>> {
//     info!("Calling route: file_retrieve {}", file_id);
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.retrieve_file(&file_id).await?;
//     Ok(Json(response))
// }

// #[axum::debug_handler]
// pub async fn file_retrieve_content(
//     app_state: Extension<Arc<AppState>>,
//     Path(file_id): Path<String>,
// ) -> Result<Json<openai::types::FileContentResponse>> {
//     info!("Calling route: file_retrieve_content {}", file_id);
//     let openai = app_state.get_openai().await;
//     let response = app_state.client.retrieve_file_content(&file_id).await?;
//     Ok(Json(response))
// }
