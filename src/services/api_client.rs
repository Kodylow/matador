use crate::{Error, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct ApiClient {
    pub client: Client,
    pub key: String,
    pub base: String,
}

impl ApiClient {
    pub fn new(key_env: &str, base: String) -> Self {
        let key = dotenv::var(key_env).unwrap_or_else(|_| panic!("{} must be set", key_env));
        let client = Client::new();
        ApiClient { client, key, base }
    }

    pub async fn send_post_request<T: Serialize + std::fmt::Debug, R: DeserializeOwned>(
        &self,
        url: &str,
        req: &T,
    ) -> Result<R> {
        let res = self
            .client
            .post(url)
            .bearer_auth(&self.key)
            .json(req)
            .send()
            .await;

        let value: R = match res {
            Ok(res) => res.json().await.unwrap(),
            Err(e) => {
                let status = e.status().unwrap();
                let text = e.to_string();
                return Err(Error::ApiError { status, text });
            }
        };
        Ok(value)
    }

    pub async fn send_get_request<R: DeserializeOwned>(&self, url: &str) -> Result<R> {
        let res = self.client.get(url).bearer_auth(&self.key).send().await;

        let value: R = match res {
            Ok(res) => res.json().await.unwrap(),
            Err(e) => {
                let status = e.status().unwrap();
                let text = e.to_string();
                return Err(Error::ApiError { status, text });
            }
        };

        Ok(value)
    }
}
