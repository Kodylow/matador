use crate::{Error, Result};
use axum::http::HeaderMap;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde_json::Value;
use std::{env, sync::Arc};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub mod types;

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

    pub async fn create_chat_completion(
        &self,
        req: types::ChatCompletionRequest,
    ) -> Result<types::ChatCompletionResponse> {
        let url = "https://api.openai.com/v1/chat/completions";
        let res = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&req)
            .send()
            .await
            .unwrap();

        let value: types::ChatCompletionResponse = res.json().await.unwrap();
        Ok(value)
    }

    pub async fn create_image(
        &self,
        req: types::ImageCreationRequest,
    ) -> Result<types::ImageCreationResponse> {
        let url = "https://api.openai.com/v1/images/generations";
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", &self.api_key).parse().unwrap(),
        );
        let res = self.client.post(url).json(&req).send().await;
        match res {
            Ok(response) => {
                println!("Response Status: {}", response.status());
                println!("Response Headers: {:?}", response.headers());
                let value: types::ImageCreationResponse = response.json().await.unwrap();
                Ok(value)
            }
            Err(e) => {
                println!("Error occurred: {:?}", e);
                panic!("Error occurred: {:?}", e);
                // Err(e.into())
            }
        }
    }
}
