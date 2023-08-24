use crate::services::api_client::ApiClient;
use crate::services::clipdrop::types::ImageCreationRequest;
use crate::Result;

use tracing::trace;

pub mod types;

#[derive(Clone)]
pub enum ClipdropEndpoints {
    ImageCreation,
    // Add other endpoints as needed
}

impl ClipdropEndpoints {
    pub fn path(&self) -> &'static str {
        match self {
            ClipdropEndpoints::ImageCreation => "/text-to-image/v1",
            // Add other endpoints as needed
        }
    }

    pub fn pricing(&self) -> u64 {
        match self {
            ClipdropEndpoints::ImageCreation => 1000,
            // Add other endpoints as needed
        }
    }
}

pub const TEXT_TO_IMAGE: &str = "/text-to-image/v1";

#[derive(Clone)]
pub struct Clipdrop {
    pub client: ApiClient,
}

impl Clipdrop {
    pub fn new() -> Self {
        trace!("Creating new Clipdrop instance");
        let client = ApiClient::new("CLIPDROP_API_KEY", "https://clipdrop-api.co".to_string());
        trace!("Clipdrop instance created");
        Clipdrop { client }
    }

    pub async fn create_image_from_text(&self, req: ImageCreationRequest) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.client.base, TEXT_TO_IMAGE);
        self.client.send_post_request(&url, &req).await
    }
}
