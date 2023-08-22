use super::LightningAddress::LightningAddress;
use axum::{
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    response::Response,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::Result;

pub async fn mw_l402<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let headers = req.headers();
    match headers.get("authorization") {
        Some(value) if value.to_str().unwrap_or("").starts_with("L402") => {
            // If the authorization header starts with "L402", proceed to the next middleware
            Ok(next.run(req).await)
        }
        _ => {
            // If the authorization header is missing or does not start with "L402", return a 402 error
            let l402 = L402::new("token".to_string()).await; // replace "token" with actual token
            let mut res = StatusCode::PAYMENT_REQUIRED.into_response();
            res.headers_mut().insert(
                "www-authenticate",
                HeaderValue::from_str(&l402.to_string()).unwrap(),
            );
            Ok(res)
        }
    }
}

struct L402 {
    token: String,
    invoice: String,
}

impl L402 {
    async fn new(token: String) -> Self {
        let lnaddress = LightningAddress::new("kodylow@getalby.com").await; // replace with actual LNADDRESS
        let invoice = lnaddress.get_invoice().await;
        L402 { token, invoice }
    }

    fn to_string(&self) -> String {
        format!(
            "L402 token=\"{}\", invoice=\"{}\"",
            self.token, self.invoice
        )
    }
}
