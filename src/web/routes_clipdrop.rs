use crate::services::clipdrop::types::ImageCreationRequest;
use crate::services::clipdrop::{self, ClipDrop, TEXT_TO_IMAGE};
use crate::{services, Error, Result};
use axum::body::{Body, Bytes};
use axum::extract::{DefaultBodyLimit, Path};
use axum::response::{IntoResponse, Response};
use axum::{routing::post, Extension, Router};
use axum::{Form, Json};
use axum_typed_multipart::TypedMultipart;
use reqwest::header::CONTENT_TYPE;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, trace};

#[derive(Clone)]
pub struct AppState {
    clipdrop: Arc<Mutex<services::clipdrop::ClipDrop>>,
}

impl AppState {
    fn new() -> Self {
        info!("Creating new AppState");
        AppState {
            clipdrop: ClipDrop::get_instance(),
        }
    }
    async fn get_clipdrop(&self) -> tokio::sync::MutexGuard<'_, services::clipdrop::ClipDrop> {
        trace!("Getting ClipDrop instance");
        self.clipdrop.lock().await
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
    let clipdrop = app_state.get_clipdrop().await;
    let image_data = clipdrop.create_image_from_text(req).await;

    match image_data {
        Ok(data) => {
            let mut res = Response::new(data.into());
            res.headers_mut()
                .insert(CONTENT_TYPE, "image/png".parse().unwrap());
            Ok(res.into_body())
        }
        Err(e) => Err(Error::ClipDropError {
            status: e.status().unwrap(),
            text: e.to_string(),
        }),
    }
}
