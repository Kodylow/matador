use axum::http::{HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use moksha_core::*;

use super::error::Result;
use crate::lightning::l402::L402;
use crate::lightning::{L402Builder, LightningAddress};

const WWW_AUTHENTICATE: &str = "www-authenticate";

pub async fn mw_l402<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let headers = req.headers();

    // Check if the x-cashu header is present
    let header = headers.get("X-Cashu").or(headers.get("x-cashu"));

    if header.is_some() {
        if handle_cashu(header.unwrap()) {
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
    let lnaddress = LightningAddress::new(
        dotenv::var("LNADDRESS")
            .expect("LNADDRESS not set")
            .as_str(),
    );
    if tokens.total_amount() < 1 {
        return false;
    }
    for token in tokens {
        let melt_url = format!("{}/melt", token.mint_url());
        let invoice = lnaddress.get_invoice(token.total_amount()).await;
        let pr = invoice.
    }

    if header == "cashu" {
        return true;
    }

    false
}
