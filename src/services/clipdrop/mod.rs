use lazy_static::lazy_static;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::trace;

use self::types::ImageCreationRequest;

pub mod types;

lazy_static! {
    static ref CLIPDROP_CLIENT: Arc<Mutex<ClipDrop>> = Arc::new(Mutex::new(ClipDrop::new()));
}

pub const TEXT_TO_IMAGE: &str = "/text-to-image/v1";

#[derive(Clone)]
pub struct ClipDrop {
    client: Client,
    api_key: String,
    api_base: String,
}

impl ClipDrop {
    fn new() -> Self {
        trace!("Creating new ClipDrop instance");
        let api_key = dotenv::var("CLIPDROP_API_KEY")
            .unwrap_or_else(|_| panic!("CLIPDROP_API_KEY must be set"));
        let client = Client::new();
        let api_base = "https://clipdrop-api.co".to_string();
        trace!("ClipDrop instance created");
        ClipDrop {
            client,
            api_key,
            api_base,
        }
    }

    pub fn get_instance() -> Arc<Mutex<ClipDrop>> {
        trace!("Getting ClipDrop instance");
        Arc::clone(&CLIPDROP_CLIENT)
    }

    pub async fn create_image_from_text(
        &self,
        req: ImageCreationRequest,
    ) -> Result<Vec<u8>, reqwest::Error> {
        trace!("Creating image from text");
        let url = format!("{}{}", self.api_base, TEXT_TO_IMAGE);
        let form = reqwest::multipart::Form::new().text("prompt", req.prompt);
        let res = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .multipart(form)
            .send()
            .await?;

        if res.status().is_success() {
            let bytes = res.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            println!("Error: {:?}", res);
            Err(res.error_for_status().unwrap_err())
        }
    }
}
