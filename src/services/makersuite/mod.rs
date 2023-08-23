use std::sync::Arc;

use crate::services::api_client::ApiClient;
use crate::services::makersuite::types::{GenerateTextRequest, GenerateTextResponse};
use crate::{Error, Result};
use lazy_static::lazy_static;
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;
use tracing::trace;

use self::types::{EmbedTextRequest, EmbedTextResponse};

pub mod types;

pub const GENERATE_TEXT: &str = "/v1beta2/models/text-bison-001:generateText";
pub const EMBED_TEXT: &str = "/v1beta2/models/embedding-gecko-001:embedText";

#[derive(Clone)]
pub struct MakerSuite {
    pub client: ApiClient,
}

impl MakerSuite {
    pub fn new() -> Self {
        trace!("Creating new MakerSuite instance");
        let client = ApiClient::new(
            "MAKERSUITE_API_KEY",
            "https://generativelanguage.googleapis.com".to_string(),
        );
        trace!("MakerSuite instance created");
        MakerSuite { client }
    }

    pub async fn generate_text(
        &self,
        model_id: &str,
        req: &GenerateTextRequest,
    ) -> Result<GenerateTextResponse> {
        let url = format!(
            "{}{}",
            self.client.base,
            GENERATE_TEXT.replace(":model_id", model_id)
        );
        self.client.send_post_request(&url, req).await
    }

    pub async fn embed_text(&self, req: EmbedTextRequest) -> Result<EmbedTextResponse> {
        trace!("Generating text embedding");
        let url = format!("{}{}", self.client.base, EMBED_TEXT);

        self.client.send_post_request(&url, &req).await
    }
}
