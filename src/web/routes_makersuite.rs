use crate::services::makersuite::{self, MakerSuite, GENERATE_TEXT};
use crate::{services, Error, Result};
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, Path};
use axum::response::Response;
use axum::Json;
use axum::{routing::post, Extension, Router};

use axum_typed_multipart::TypedMultipart;
use reqwest::header::CONTENT_TYPE;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, trace};

#[derive(Clone)]
pub struct AppState {
    makersuite: Arc<Mutex<services::makersuite::MakerSuite>>,
}

impl AppState {
    fn new() -> Self {
        info!("Creating new AppState");
        AppState {
            makersuite: MakerSuite::get_instance(),
        }
    }
    async fn get_makersuite(
        &self,
    ) -> tokio::sync::MutexGuard<'_, services::makersuite::MakerSuite> {
        trace!("Getting MakerSuite instance");
        self.makersuite.lock().await
    }
}

pub fn routes() -> Router {
    info!("Setting up routes");
    let app_state = Arc::new(AppState::new());
    Router::new()
        .route(GENERATE_TEXT, post(generate_text))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .layer(Extension(app_state))
}

#[axum::debug_handler]
pub async fn generate_text(
    app_state: Extension<Arc<AppState>>,
    Path(model_id): Path<String>,
    Json(req): Json<makersuite::types::GenerateTextRequest>,
) -> Result<Json<makersuite::types::GenerateTextResponse>> {
    info!("Calling route: generate_text");
    let makersuite = app_state.get_makersuite().await;
    println!("model_id: {}", model_id);
    let text_response = makersuite.generate_text(model_id.as_str(), req).await?;

    Ok(Json(text_response))
}
