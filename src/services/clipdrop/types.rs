use axum_typed_multipart::TryFromMultipart;
use serde::{Deserialize, Serialize};

#[derive(Debug, TryFromMultipart, Deserialize, Serialize)]
pub struct ImageCreationRequest {
    pub prompt: String,
}
