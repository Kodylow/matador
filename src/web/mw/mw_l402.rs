use axum::http::{HeaderName, HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use moksha_core::model::{Proofs, TokenV3};
use moksha_core::*;
use serde::{Deserialize, Serialize};

use super::error::Result;
use crate::config::config::config;
use crate::lightning::l402::L402;
use crate::lightning::{Cashu402Builder, L402Builder, LightningAddress};

const WWW_AUTHENTICATE: &str = "www-authenticate";
const X_CASHU: &str = "x-cashu";

pub async fn mw_402<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let headers = req.headers().clone();

    // X-Cashu handling
    let cashu_header = headers.get("X-Cashu").or(headers.get("x-cashu"));
    if let Some(header) = cashu_header {
        return handle_cashu_header(header, req, next).await;
    }

    // L402 handling
    let auth_header = headers
        .get("Authorization")
        .or(headers.get("authorization"));
    if let Some(header) = auth_header {
        return handle_auth_header(header, req, next).await;
    }

    // If the authorization header is missing or does not start with "L402", return
    // a 402 error
    handle_missing_auth_header(req).await
}

async fn handle_cashu_header<B>(
    header: &HeaderValue,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    match handle_cashu(header).await {
        true => Ok(next.run(req).await),
        false => generate_payment_required_response().await,
    }
}

async fn handle_auth_header<B>(
    header: &HeaderValue,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let l402 = L402::from_auth_header(header.to_str().unwrap())?;
    match l402.is_valid().unwrap() {
        true => Ok(next.run(req).await),
        false => generate_payment_required_response().await,
    }
}

async fn handle_missing_auth_header<B>(req: Request<B>) -> Result<Response> {
    generate_payment_required_response().await
}

async fn generate_payment_required_response() -> Result<Response> {
    let mut res = StatusCode::PAYMENT_REQUIRED.into_response();
    let l402 = L402Builder::new().build().await?;
    res.headers_mut().insert(
        WWW_AUTHENTICATE,
        HeaderValue::from_str(&l402.to_authenticate_string()).unwrap(),
    );

    let cashu402 = Cashu402Builder::new().amount(3).build().await?;
    res.headers_mut().insert(
        "x-cashu",
        HeaderValue::from_str(&cashu402.to_authenticate_string()).unwrap(),
    );
    Ok(res)
}

async fn handle_cashu(header: &HeaderValue) -> bool {
    let header = header.to_str().unwrap().to_string();
    let tokens = TokenV3::deserialize(header).unwrap();
    let mint_url = tokens.tokens.get(0).unwrap().clone().mint.unwrap();
    println!("mint_url: {}", mint_url);
    println!("cashu_mint_url: {}", config().CASHU_MINT_URL);
    if mint_url != config().CASHU_MINT_URL {
        return false;
    }
    let mint_url = format!("{}/melt", mint_url.to_string());
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
    let proofs = tokens.proofs();
    let body = serde_json::to_string(&PostMeltRequest { pr, proofs }).unwrap();

    let client = reqwest::Client::new();
    let req = client
        .post(mint_url)
        .header("Content-Type", "application/json")
        .body(body);

    let res = req.send().await.unwrap();

    // println!("res: {:?}", res.text().await.unwrap());

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
