use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

const BASE_URL: &str = "http://localhost:8080/openai/v1";

#[tokio::test]
async fn test_list_models() -> Result<()> {
    let client = Client::new();
    let _resp: Value = client
        .get(&format!("{}/models", BASE_URL))
        .send()
        .await?
        .json()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_retrieve_model() -> Result<()> {
    let client = Client::new();
    let _resp: Value = client
        .get(&format!("{}/models/text-davinci-003", BASE_URL))
        .send()
        .await?
        .json()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_chat_completion_create() -> Result<()> {
    let client = Client::new();
    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            { "role": "system", "content": "You are a helpful assistant." },
            { "role": "user", "content": "Hello!" }
        ]
    });
    let _resp: Value = client
        .post(&format!("{}/chat/completions", BASE_URL))
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_image_generations() -> Result<()> {
    let client = Client::new();
    let body = json!({
        "prompt": "A cute baby sea otter",
        "n": 2,
        "size": "1024x1024"
    });
    let _resp: Value = client
        .post(&format!("{}/images/generations", BASE_URL))
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_embeddings_create() -> Result<()> {
    let client = Client::new();
    let body = json!({
        "input": "The food was delicious and the waiter...",
        "model": "text-embedding-ada-002"
    });
    let _resp: Value = client
        .post(&format!("{}/embeddings", BASE_URL))
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    Ok(())
}
