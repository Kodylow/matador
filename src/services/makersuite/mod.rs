use crate::services::api_client::ApiClient;
use crate::services::makersuite::types::{GenerateTextRequest, GenerateTextResponse};
use crate::Result;

use tracing::trace;

use self::types::{EmbedTextRequest, EmbedTextResponse};

pub mod types;

pub enum MakerSuiteEndpoints {
    GenerateText,
    EmbedText,
    // Add other endpoints as needed
}

impl MakerSuiteEndpoints {
    pub fn path(&self) -> &'static str {
        match self {
            MakerSuiteEndpoints::GenerateText => "/v1beta2/models/text-bison-001:generateText",
            MakerSuiteEndpoints::EmbedText => "/v1beta2/models/embedding-gecko-001:embedText",
            // Add other endpoints as needed
        }
    }
}

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
        req: GenerateTextRequest,
    ) -> Result<GenerateTextResponse> {
        println!("In generate_text");
        let url = format!(
            "{}{}",
            self.client.base,
            MakerSuiteEndpoints::GenerateText
                .path()
                .replace(":model_id", model_id)
        );
        self.client
            .send_post_request_with_key_query(&url, &req)
            .await
    }

    pub async fn embed_text(&self, req: EmbedTextRequest) -> Result<EmbedTextResponse> {
        trace!("Generating text embedding");
        let url = format!(
            "{}{}",
            self.client.base,
            MakerSuiteEndpoints::EmbedText.path()
        );

        self.client
            .send_post_request_with_key_query(&url, &req)
            .await
    }
}
