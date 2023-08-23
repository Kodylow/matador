use crate::services::api_client::ApiClient;
use crate::services::clipdrop::types::ImageCreationRequest;
use crate::Result;

use tracing::trace;

pub mod types;

pub const TEXT_TO_IMAGE: &str = "/text-to-image/v1";

#[derive(Clone)]
pub struct ClipDrop {
    pub client: ApiClient,
}

impl ClipDrop {
    pub fn new() -> Self {
        trace!("Creating new ClipDrop instance");
        let client = ApiClient::new("CLIPDROP_API_KEY", "https://clipdrop-api.co".to_string());
        trace!("ClipDrop instance created");
        ClipDrop { client }
    }

    pub async fn create_image_from_text(&self, req: ImageCreationRequest) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.client.base, TEXT_TO_IMAGE);
        self.client.send_post_request(&url, &req).await
    }
}
