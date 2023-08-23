use crate::services::clipdrop::types::ImageCreationRequest;
use crate::services::clipdrop::{self, ClipDrop, TEXT_TO_IMAGE};
use crate::{services, Error, Result};
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, Path};
use axum::response::Response;
use axum::Json;
use axum::{routing::post, Extension, Router};

use axum_typed_multipart::TypedMultipart;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, trace};

#[derive(Clone)]
pub struct AppState {
    client: ClipDrop,
}

impl AppState {
    fn new() -> Self {
        info!("Creating new AppState");
        AppState {
            client: ClipDrop::new(),
        }
    }
}

pub fn routes() -> Router {
    info!("Setting up routes");
    let app_state = Arc::new(AppState::new());
    Router::new()
        .route(TEXT_TO_IMAGE, post(image_create_from_text))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .layer(Extension(app_state))
}

#[axum::debug_handler]
pub async fn image_create_from_text(
    app_state: Extension<Arc<AppState>>,
    TypedMultipart(req): TypedMultipart<ImageCreationRequest>,
) -> Result<Bytes> {
    info!("Calling route: image_create_from_text");
    let image_response = app_state.client.create_image_from_text(req).await;

    match image_response {
        Ok(bytes) => Ok(bytes.into()),
        Err(e) => Err(Error::ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            text: "Error creating image from text".to_string(),
        }),
    }
}
