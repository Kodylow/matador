use axum::http::{HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use moksha_core::model::{Proofs, TokenV3};
use moksha_core::*;
use serde::{Deserialize, Serialize};

use super::error::Result;
use crate::lightning::l402::L402;
use crate::lightning::{L402Builder, LightningAddress};

const WWW_AUTHENTICATE: &str = "www-authenticate";

pub async fn mw_l402<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let headers = req.headers();

    // Check if the x-cashu header is present
    let header = headers.get("X-Cashu").or(headers.get("x-cashu"));

    if header.is_some() {
        if handle_cashu(header.unwrap()).await {
            return Ok(next.run(req).await);
        } else {
            let mut res = StatusCode::PAYMENT_REQUIRED.into_response();
            let l402 = L402Builder::new().build().await?;
            res.headers_mut().insert(
                WWW_AUTHENTICATE,
                HeaderValue::from_str(&l402.to_authenticate_string()).unwrap(),
            );

            return Ok(res);
        }
    }

    // Check if the authorization header is present, handle Authorization and
    // authorization
    let header = headers
        .get("Authorization")
        .or(headers.get("authorization"));

    match header {
        Some(header) => {
            let l402 = L402::from_auth_header(header.to_str().unwrap())?;
            if l402.is_valid().unwrap() {
                // If the token is valid, call the next middleware
                Ok(next.run(req).await)
            } else {
                // If the token is invalid, return a 402 error
                let mut res = StatusCode::PAYMENT_REQUIRED.into_response();
                let l402 = L402Builder::new().build().await?;
                res.headers_mut().insert(
                    WWW_AUTHENTICATE,
                    HeaderValue::from_str(&l402.to_authenticate_string()).unwrap(),
                );

                Ok(res)
            }
        }
        _ => {
            // If the authorization header is missing or does not start with "L402", return
            // a 402 error
            let l402 = L402Builder::new().build().await?;
            let mut res = StatusCode::PAYMENT_REQUIRED.into_response();
            res.headers_mut().insert(
                WWW_AUTHENTICATE,
                HeaderValue::from_str(&l402.to_authenticate_string()).unwrap(),
            );
            Ok(res)
        }
    }
}

async fn handle_cashu(header: &HeaderValue) -> bool {
    let header = header.to_str().unwrap().to_string();
    let tokens = TokenV3::deserialize(header).unwrap();
    if tokens.total_amount() < 1 {
        return false;
    }
    let mint_url = tokens.tokens.get(0).unwrap().clone().mint.unwrap();
    let mint_url = mint_url.join("melt").unwrap();
    let lnaddress = LightningAddress::new(
        dotenv::var("LNADDRESS")
            .expect("LNADDRESS not set")
            .as_str(),
    )
    .await;
    let pr = lnaddress
        .get_invoice(((tokens.total_amount() - 2) * 1000).try_into().unwrap())
        .await
        .into_signed_raw()
        .to_string();
    println!("pr: {}", pr);
    let proofs = tokens.proofs();
    let body = serde_json::to_string(&PostMeltRequest { pr, proofs }).unwrap();

    let client = reqwest::Client::new();
    let req = client
        .post(mint_url)
        .header("Content-Type", "application/json")
        .body(body);

    let res = req.send().await.unwrap();

    if res.status().is_success() {
        return true;
    }

    false
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMeltRequest {
    pub proofs: Proofs,
    pub pr: String,
}
