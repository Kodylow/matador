use crate::{services::api_client::ApiClient, Error, Result};

use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tracing::trace;

use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

pub mod types;

pub const MODEL_LIST: &str = "/v1/models";
pub const MODEL_RETRIEVE: &str = "/v1/models/:model_id";
pub const CHAT_COMPLETIONS: &str = "/v1/chat/completions";
pub const IMAGE_GENERATIONS: &str = "/v1/images/generations";
pub const EMBEDDINGS: &str = "/v1/embeddings";
// pub const IMAGE_EDITS: &str = "/v1/images/edits";
// pub const IMAGE_VARIATIONS: &str = "/v1/images/variations";
// pub const TRANSCRIPTIONS: &str = "/v1/audio/transcriptions";
// pub const TRANSLATIONS: &str = "/v1/audio/translations";
// pub const FILES: &str = "/v1/files";
// pub const FILE: &str = "/v1/files/:file_id";
// pub const FILE_CONTENT: &str = "/v1/files/:file_id/content";

#[derive(Clone)]
pub struct OpenAI {
    client: ApiClient,
}

impl OpenAI {
    pub fn new() -> Self {
        trace!("Creating new OpenAI instance");
        let client = ApiClient::new("OPENAI_API_KEY", "https://api.openai.com".to_string());
        trace!("OpenAI instance created");
        OpenAI { client }
    }

    pub async fn model_list(&self) -> Result<Value> {
        trace!("Fetching model list");
        let url = format!("{}{}", self.client.base, MODEL_LIST);
        self.client.send_get_request(&url).await
    }

    pub async fn model_retrieve(&self, model_id: &str) -> Result<Value> {
        trace!("Retrieving model {}", model_id);
        let url = format!(
            "{}{}",
            self.client.base,
            MODEL_RETRIEVE.replace(":model_id", model_id)
        );
        self.client.send_get_request(&url).await
    }

    pub async fn create_chat_completion(
        &self,
        req: types::ChatCompletionRequest,
    ) -> Result<types::ChatCompletionResponse> {
        trace!("Creating chat completion");
        let url = format!("{}{}", self.client.base, CHAT_COMPLETIONS);
        self.client.send_post_request(&url, &req).await
    }

    pub async fn create_embeddings(
        &self,
        req: types::EmbeddingRequest,
    ) -> Result<types::EmbeddingResponse> {
        trace!("Creating embeddings");
        let url = format!("{}{}", self.client.base, EMBEDDINGS);
        self.client.send_post_request(&url, &req).await
    }

    pub async fn create_image(
        &self,
        req: types::ImageCreationRequest,
    ) -> Result<types::ImageResponse> {
        trace!("Creating image");
        let url = format!("{}{}", self.client.base, IMAGE_GENERATIONS);
        self.client.send_post_request(&url, &req).await
    }

    // pub async fn edit_image(&self, req: types::ImageEditRequest) -> Result<types::ImageResponse> {
    //     trace!("Editing image");
    //     let url = format!("{}{}", self.api_base, IMAGE_EDITS);
    //     self.send_post_request(&url, &req).await
    // }

    // pub async fn vary_image(
    //     &self,
    //     req: types::ImageVariationRequest,
    // ) -> Result<types::ImageResponse> {
    //     trace!("Creating image variation");
    //     let url = format!("{}{}", self.api_base, IMAGE_VARIATIONS);
    //     self.send_post_request(&url, &req).await
    // }

    // pub async fn create_transcription(
    //     &self,
    //     req: types::AudioRequest,
    // ) -> Result<types::AudioResponse> {
    //     trace!("Creating transcription");
    //     let url = format!("{}{}", self.api_base, TRANSCRIPTIONS);
    //     self.send_post_request(&url, &req).await
    // }

    // pub async fn create_translation(
    //     &self,
    //     req: types::AudioRequest,
    // ) -> Result<types::AudioResponse> {
    //     trace!("Creating translation");
    //     let url = format!("{}{}", self.api_base, TRANSLATIONS);
    //     self.send_post_request(&url, &req).await
    // }

    // pub async fn list_files(&self) -> Result<types::FileListResponse> {
    //     trace!("Listing files");
    //     let url = format!("{}{}", self.api_base, FILES);
    //     self.send_get_request(&url).await
    // }

    // pub async fn upload_file(&self, req: types::FileUploadRequest) -> Result<types::FileResponse> {
    //     trace!("Uploading file");
    //     let url = format!("{}{}", self.api_base, FILES);
    //     self.send_post_request(&url, &req).await
    // }

    // pub async fn delete_file(&self, file_id: &str) -> Result<types::FileDeletionResponse> {
    //     trace!("Deleting file {}", file_id);
    //     let url = format!("{}{}", self.api_base, FILE.replace(":file_id", file_id));
    //     self.send_delete_request(&url).await
    // }

    // pub async fn retrieve_file(&self, file_id: &str) -> Result<types::FileResponse> {
    //     trace!("Retrieving file {}", file_id);
    //     let url = format!("{}{}", self.api_base, FILE.replace(":file_id", file_id));
    //     self.send_get_request(&url).await
    // }

    // pub async fn retrieve_file_content(&self, file_id: &str) -> Result<types::FileContentResponse> {
    //     trace!("Retrieving file content {}", file_id);
    //     let url = format!(
    //         "{}{}",
    //         self.api_base,
    //         FILE_CONTENT.replace(":file_id", file_id)
    //     );
    //     self.send_get_request(&url).await
    // }

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

    // async fn send_delete_request<R: DeserializeOwned>(&self, url: &str) -> Result<R> {
    //     trace!("Sending DELETE request to {}", url);
    //     let res = self
    //         .client
    //         .delete(url)
    //         .bearer_auth(&self.api_key)
    //         .send()
    //         .await;

    //     match res {
    //         Ok(r) => {
    //             trace!("DELETE request successful");
    //             let value: R = r.json().await.unwrap();
    //             Ok(value)
    //         }
    //         Err(e) => {
    //             trace!("DELETE request failed");
    //             let status = e.status().unwrap();
    //             let text = e.to_string();
    //             Err(Error::OpenAIError { status, text })
    //         }
    //     }
    // }
}
