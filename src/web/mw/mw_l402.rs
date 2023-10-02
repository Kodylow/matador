use crate::lightning::L402Builder;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};

use super::error::Result;

const WWW_AUTHENTICATE: &str = "www-authenticate";

pub async fn mw_l402<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let headers = req.headers();

    // Check if the authorization header is present, handle Authorization and authorization
    let header = headers
        .get("Authorization")
        .or(headers.get("authorization"));

    match header {
        Some(header) => {
            let l402 = L402Builder::new().build().await?;
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
            // If the authorization header is missing or does not start with "L402", return a 402 error
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
