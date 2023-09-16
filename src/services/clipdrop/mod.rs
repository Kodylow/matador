use crate::services::api_client::ApiClient;
use crate::services::clipdrop::types::ImageCreationRequest;
use crate::{Error, Result};

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
        trace!("Creating image from text");
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.client.base, TEXT_TO_IMAGE);
        let form = reqwest::multipart::Form::new().text("prompt", req.prompt);
        let res = client
            .post(&url)
            .header("x-api-key", &self.client.key)
            .multipart(form)
            .send()
            .await
            .unwrap();

        if res.status().is_success() {
            let bytes = res.bytes().await.unwrap();
            Ok(bytes.to_vec())
        } else {
            println!("Error: {:?}", res);
            Err(Error::ApiError {
                status: res.status(),
                text: "Error creating image from text".to_string(),
            })
        }
    }
}
