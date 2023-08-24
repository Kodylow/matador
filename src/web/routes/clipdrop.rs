use crate::services::clipdrop::types::ImageCreationRequest;
use crate::services::clipdrop::{Clipdrop, ClipdropEndpoints};
use crate::{Error, Result};
use axum::body::Bytes;
use axum::extract::DefaultBodyLimit;

use axum::{routing::post, Extension, Router};

use axum_typed_multipart::TypedMultipart;

use reqwest::StatusCode;
use std::sync::Arc;

use tracing::info;

#[derive(Clone)]
pub struct AppState {
    client: Clipdrop,
}

impl AppState {
    fn new() -> Self {
        info!("Creating new AppState");
        AppState {
            client: Clipdrop::new(),
        }
    }
}

pub fn routes() -> Router {
    info!("Setting up routes");
    let app_state = Arc::new(AppState::new());
    Router::new()
        .route(
            ClipdropEndpoints::ImageCreation.path(),
            post(image_create_from_text),
        )
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
        Err(_e) => Err(Error::ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            text: "Error creating image from text".to_string(),
        }),
    }
}
