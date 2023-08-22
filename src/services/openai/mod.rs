use crate::{Error, Result};

use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tracing::trace;

use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

pub mod types;

lazy_static! {
    static ref OPENAI_CLIENT: Arc<Mutex<OpenAI>> = Arc::new(Mutex::new(OpenAI::new()));
}

pub const MODEL_LIST: &str = "/v1/models";
pub const MODEL_RETRIEVE: &str = "/v1/models/:model_id";
pub const CHAT_COMPLETIONS: &str = "/v1/chat/completions";
pub const IMAGE_GENERATIONS: &str = "/v1/images/generations";
pub const IMAGE_EDITS: &str = "/v1/images/edits";
pub const IMAGE_VARIATIONS: &str = "/v1/images/variations";
pub const EMBEDDINGS: &str = "/v1/embeddings";
pub const TRANSCRIPTIONS: &str = "/v1/audio/transcriptions";
pub const TRANSLATIONS: &str = "/v1/audio/translations";
pub const FILES: &str = "/v1/files";
pub const FILE: &str = "/v1/files/:file_id";
pub const FILE_CONTENT: &str = "/v1/files/:file_id/content";

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
        let api_base = "https://api.openai.com".to_string();
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
        let url = format!(
            "{}{}",
            self.api_base,
            MODEL_RETRIEVE.replace(":model_id", model_id)
        );
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
    ) -> Result<types::ImageResponse> {
        trace!("Creating image");
        let url = format!("{}{}", self.api_base, IMAGE_GENERATIONS);
        self.send_post_request(&url, &req).await
    }

    pub async fn edit_image(&self, req: types::ImageEditRequest) -> Result<types::ImageResponse> {
        trace!("Editing image");
        let url = format!("{}{}", self.api_base, IMAGE_EDITS);
        self.send_post_request(&url, &req).await
    }

    pub async fn vary_image(
        &self,
        req: types::ImageVariationRequest,
    ) -> Result<types::ImageResponse> {
        trace!("Creating image variation");
        let url = format!("{}{}", self.api_base, IMAGE_VARIATIONS);
        self.send_post_request(&url, &req).await
    }

    pub async fn create_embeddings(
        &self,
        req: types::EmbeddingRequest,
    ) -> Result<types::EmbeddingResponse> {
        trace!("Creating embeddings");
        let url = format!("{}{}", self.api_base, EMBEDDINGS);
        self.send_post_request(&url, &req).await
    }

    pub async fn create_transcription(
        &self,
        req: types::AudioRequest,
    ) -> Result<types::AudioResponse> {
        trace!("Creating transcription");
        let url = format!("{}{}", self.api_base, TRANSCRIPTIONS);
        self.send_post_request(&url, &req).await
    }

    pub async fn create_translation(
        &self,
        req: types::AudioRequest,
    ) -> Result<types::AudioResponse> {
        trace!("Creating translation");
        let url = format!("{}{}", self.api_base, TRANSLATIONS);
        self.send_post_request(&url, &req).await
    }

    pub async fn list_files(&self) -> Result<types::FileListResponse> {
        trace!("Listing files");
        let url = format!("{}{}", self.api_base, FILES);
        self.send_get_request(&url).await
    }

    pub async fn upload_file(&self, req: types::FileUploadRequest) -> Result<types::FileResponse> {
        trace!("Uploading file");
        let url = format!("{}{}", self.api_base, FILES);
        self.send_post_request(&url, &req).await
    }

    pub async fn delete_file(&self, file_id: &str) -> Result<types::FileDeletionResponse> {
        trace!("Deleting file {}", file_id);
        let url = format!("{}{}", self.api_base, FILE.replace(":file_id", file_id));
        self.send_delete_request(&url).await
    }

    pub async fn retrieve_file(&self, file_id: &str) -> Result<types::FileResponse> {
        trace!("Retrieving file {}", file_id);
        let url = format!("{}{}", self.api_base, FILE.replace(":file_id", file_id));
        self.send_get_request(&url).await
    }

    pub async fn retrieve_file_content(&self, file_id: &str) -> Result<types::FileContentResponse> {
        trace!("Retrieving file content {}", file_id);
        let url = format!(
            "{}{}",
            self.api_base,
            FILE_CONTENT.replace(":file_id", file_id)
        );
        self.send_get_request(&url).await
    }

    async fn send_get_request<R: DeserializeOwned>(&self, url: &str) -> Result<R> {
        trace!("Sending GET request to {}", url);
        let res = self.client.get(url).bearer_auth(&self.api_key).send().await;

        let value: R = match res {
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

    async fn send_post_request<T: Serialize + std::fmt::Debug, R: DeserializeOwned>(
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

        println!("Request: {:?}", req);
        // println!("Response: {:?}", res);
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

    // async fn send_post_request_with_form<T: DeserializeOwned>(
    //     &self,
    //     url: &str,
    //     form: axum::extract::multipart::,
    // ) -> Result<T> {
    //     trace!("Sending POST request to {}", url);
    //     let res = self
    //         .client
    //         .post(url)
    //         .bearer_auth(&self.api_key)
    //         .multipart(form)
    //         .send()
    //         .await;

    //     let value: T = match res {
    //         Ok(res) => {
    //             trace!("POST request successful");
    //             res.json().await.unwrap()
    //         }
    //         Err(e) => {
    //             trace!("POST request failed");
    //             let status = e.status().unwrap();
    //             let text = e.to_string();
    //             return Err(Error::OpenAIError { status, text });
    //         }
    //     };
    //     Ok(value)
    // }

    async fn send_delete_request<R: DeserializeOwned>(&self, url: &str) -> Result<R> {
        trace!("Sending DELETE request to {}", url);
        let res = self
            .client
            .delete(url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(r) => {
                trace!("DELETE request successful");
                let value: R = r.json().await.unwrap();
                Ok(value)
            }
            Err(e) => {
                trace!("DELETE request failed");
                let status = e.status().unwrap();
                let text = e.to_string();
                Err(Error::OpenAIError { status, text })
            }
        }
    }
}
