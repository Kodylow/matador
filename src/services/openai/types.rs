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
    completion_tokens: i32,
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
