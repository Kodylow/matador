use crate::{Error, Result};

use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, trace};

use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

pub mod types;

lazy_static! {
    static ref OPENAI_CLIENT: Arc<Mutex<OpenAI>> = Arc::new(Mutex::new(OpenAI::new()));
}

pub const MODEL_LIST: &str = "/v1/models";
pub const CHAT_COMPLETIONS: &str = "/v1/chat/completions";
pub const MODEL_RETRIEVE: &str = "/v1/models/:model_id";
pub const IMAGE_GENERATIONS: &str = "/v1/images/generations";

#[derive(Clone)]
pub struct OpenAI {
    client: Client,
    api_key: String,
    api_base: String,
}

impl OpenAI {
    fn new() -> Self {
        trace!("Creating new OpenAI instance");
        let api_key =
            dotenv::var("OPENAI_API_KEY").unwrap_or_else(|_| panic!("OPENAI_API_KEY must be set"));
        let client = Client::new();
        let api_base = "https://api.openai.com/v1".to_string();
        trace!("OpenAI instance created");
        OpenAI {
            client,
            api_key,
            api_base,
        }
    }

    pub fn get_instance() -> Arc<Mutex<OpenAI>> {
        trace!("Getting OpenAI instance");
        Arc::clone(&OPENAI_CLIENT)
    }

    pub async fn model_list(&self) -> Result<Value> {
        trace!("Fetching model list");
        let url = format!("{}{}", self.api_base, MODEL_LIST);
        self.send_get_request(&url).await
    }

    pub async fn model_retrieve(&self, model_id: &str) -> Result<Value> {
        trace!("Retrieving model {}", model_id);
        let url = format!("{}{}{}", self.api_base, MODEL_RETRIEVE, model_id);
        self.send_get_request(&url).await
    }

    pub async fn create_chat_completion(
        &self,
        req: types::ChatCompletionRequest,
    ) -> Result<types::ChatCompletionResponse> {
        trace!("Creating chat completion");
        let url = format!("{}{}", self.api_base, CHAT_COMPLETIONS);
        self.send_post_request(&url, &req).await
    }

    pub async fn create_image(
        &self,
        req: types::ImageCreationRequest,
    ) -> Result<types::ImageCreationResponse> {
        trace!("Creating image");
        let url = format!("{}{}", self.api_base, IMAGE_GENERATIONS);
        self.send_post_request(&url, &req).await
    }

    async fn send_get_request(&self, url: &str) -> Result<Value> {
        trace!("Sending GET request to {}", url);
        let res = self.client.get(url).bearer_auth(&self.api_key).send().await;

        let value: Value = match res {
            Ok(res) => {
                trace!("GET request successful");
                res.json().await.unwrap()
            }
            Err(e) => {
                trace!("GET request failed");
                let status = e.status().unwrap();
                let text = e.to_string();
                return Err(Error::OpenAIError { status, text });
            }
        };

        Ok(value)
    }

    async fn send_post_request<T: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        req: &T,
    ) -> Result<R> {
        trace!("Sending POST request to {}", url);
        let res = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(req)
            .send()
            .await;

        let value: R = match res {
            Ok(res) => {
                trace!("POST request successful");
                res.json().await.unwrap()
            }
            Err(e) => {
                trace!("POST request failed");
                let status = e.status().unwrap();
                let text = e.to_string();
                return Err(Error::OpenAIError { status, text });
            }
        };
        Ok(value)
    }
}
