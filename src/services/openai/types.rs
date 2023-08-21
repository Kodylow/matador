use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    functions: Option<Vec<Function>>,
    function_call: Option<FunctionCall>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<i32>,
    pub stream: Option<bool>,
    stop: Option<Vec<String>>,
    max_tokens: Option<i32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    logit_bias: Option<HashMap<String, f32>>,
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    name: String,
    description: Option<String>,
    parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionCall {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    index: i32,
    message: Message,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: Option<i32>,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionChunk {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<ChoiceChunk>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceChunk {
    index: i32,
    delta: Delta,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCreationRequest {
    prompt: String,
    n: Option<i32>,
    size: Option<String>,
    response_format: Option<String>,
    user: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ImageResponse {
    created: String,
    data: Vec<ImageData>,
}

#[derive(Deserialize, Serialize)]
pub struct ImageData {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageEditRequest {
    image: String,
    mask: Option<String>,
    prompt: String,
    n: Option<i32>,
    size: Option<String>,
    response_format: Option<String>,
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageVariationRequest {
    image: String,
    n: Option<i32>,
    size: Option<String>,
    response_format: Option<String>,
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: Vec<String>,
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingObject>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddingObject {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AudioRequest {
    pub file: String,
    pub model: String,
    pub prompt: Option<String>,
    pub response_format: Option<String>,
    pub temperature: Option<f32>,
    pub language: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AudioResponse {
    pub text: String,
}

// src/services/openai/types.rs

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListResponse {
    pub data: Vec<FileResponse>,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResponse {
    pub id: String,
    pub object: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
    pub status: String,
    pub status_details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUploadRequest {
    pub file: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileDeletionResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContentResponse {
    pub content: String,
}
