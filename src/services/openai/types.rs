use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    // Add other fields as per your requirements
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
