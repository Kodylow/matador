use crate::{Error, Result};
use reqwest::Client;
use serde_json::Value;
use std::{env, sync::Arc};

use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    static ref OPENAI_CLIENT: Arc<Mutex<OpenAI>> = Arc::new(Mutex::new(OpenAI::new()));
}

#[derive(Clone)]
pub struct OpenAI {
    client: Client,
    api_key: String,
}

impl OpenAI {
    fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let client = Client::new();
        OpenAI { client, api_key }
    }

    pub fn get_instance() -> Arc<Mutex<OpenAI>> {
        Arc::clone(&OPENAI_CLIENT)
    }

    pub async fn model_list(&self) -> Result<Value> {
        let url = "https://api.openai.com/v1/models";
        self.send_request(url).await
    }

    pub async fn model_retrieve(&self, model_id: &str) -> Result<Value> {
        let url = format!("https://api.openai.com/v1/models/{}", model_id);
        self.send_request(&url).await
    }

    async fn send_request(&self, url: &str) -> Result<Value> {
        let res = self
            .client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .unwrap();

        let value: Value = res.json().await.unwrap();
        Ok(value)
    }
}
