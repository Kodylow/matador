use super::lightning_address::LightningAddress;
use crate::error::Result;
use axum::{
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    response::Response,
};
use futhark::{Restriction, Rune};
use lightning_invoice::Bolt11Invoice;

pub async fn mw_l402<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let headers = req.headers();
    match headers.get("authorization") {
        Some(value) if value.to_str().unwrap_or("").starts_with("L402") => {
            // If the authorization header starts with "L402", proceed to the next middleware
            Ok(next.run(req).await)
        }
        _ => {
            // If the authorization header is missing or does not start with "L402", return a 402 error
            let l402 = L402::new().await; // replace "token" with actual token
            let mut res = StatusCode::PAYMENT_REQUIRED.into_response();
            res.headers_mut().insert(
                "www-authenticate",
                HeaderValue::from_str(&l402.to_authenticate_string()).unwrap(),
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
    async fn new() -> Self {
        let lnaddress = LightningAddress::new(dotenv::var("LNADDRESS").unwrap().as_str()).await;
        let invoice: Bolt11Invoice = lnaddress.get_invoice().await;
        let payment_hash = invoice.payment_hash();
        let token = build_rune(payment_hash.to_string());
        let invoice = invoice.to_string();
        L402 { token, invoice }
    }

    fn to_authenticate_string(&self) -> String {
        format!(
            "L402 token=\"{}\", invoice=\"{}\"",
            self.token, self.invoice
        )
    }
}

fn build_rune(payment_hash: String) -> String {
    let secret = [0u8; 16];
    let mut mr = Rune::new_master_rune(&secret, vec![], None, None).unwrap();
    let (res, _) = Restriction::decode(&format!("payment_hash={}", payment_hash), false).unwrap();
    mr.add_restriction(res);
    let rune = mr.to_base64();

    rune
}
