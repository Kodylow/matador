use std::sync::Arc;

use crate::services::makersuite::types::{GenerateTextRequest, GenerateTextResponse};
use crate::{Error, Result};
use lazy_static::lazy_static;
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;
use tracing::trace;

pub mod types;

lazy_static! {
    static ref MAKERSUITE_CLIENT: Arc<Mutex<MakerSuite>> = Arc::new(Mutex::new(MakerSuite::new()));
}

pub const GENERATE_TEXT: &str = "/v1beta2/models/:model_id";

pub struct MakerSuite {
    api_key: String,
    api_base: String,
    client: reqwest::Client,
}

impl MakerSuite {
    fn new() -> Self {
        trace!("Creating new MakerSuite instance");
        let api_key = dotenv::var("MAKERSUITE_API_KEY")
            .unwrap_or_else(|_| panic!("PALM_API_KEY must be set"));
        let client = Client::new();
        let api_base = "https://generativelanguage.googleapis.com".to_string();
        trace!("MakerSuite instance created");
        MakerSuite {
            client,
            api_key,
            api_base,
        }
    }

    pub fn get_instance() -> Arc<Mutex<MakerSuite>> {
        trace!("Getting MakerSuite instance");
        Arc::clone(&MAKERSUITE_CLIENT)
    }

    pub async fn generate_text(
        &self,
        model_id: &str,
        req: GenerateTextRequest,
    ) -> Result<GenerateTextResponse> {
        trace!("Generating text from model: {}", model_id);
        let url = format!(
            "{}{}",
            self.api_base,
            GENERATE_TEXT.replace(":model_id", model_id)
        );

        self.send_post_request(&url, &req).await
    }

    async fn send_post_request<T: Serialize + std::fmt::Debug, R: DeserializeOwned>(
        &self,
        url: &str,
        req: &T,
    ) -> Result<R> {
        trace!("Sending POST request to {}", url);
        let url_with_key = format!("{}?key={}", url, self.api_key);
        println!("url_with_key: {}", url_with_key);
        let res = self.client.post(&url_with_key).json(req).send().await;

        let value: R = match res {
            Ok(res) => {
                trace!("POST request successful");
                res.json().await.unwrap()
            }
            Err(e) => {
                trace!("POST request failed");
                let status = e.status().unwrap();
                let text = e.to_string();
                return Err(Error::MakerSuiteError { status, text });
            }
        };
        Ok(value)
    }
}
