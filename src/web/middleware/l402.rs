use super::lightning_address::LightningAddress;
use crate::error::Error;
use axum::{
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    response::Response,
};
use lightning_invoice::Bolt11Invoice;
use macaroon::{Format, Macaroon, MacaroonKey, Verifier};
use sha2::{Digest, Sha256};
use std::env;

pub async fn mw_l402<B>(req: Request<B>, next: Next<B>) -> crate::Result<Response> {
    let headers = req.headers();

    // Check if the authorization header is present, handle Authorization and authorization
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
                let l402 = L402::new().await;
                res.headers_mut().insert(
                    "www-authenticate",
                    HeaderValue::from_str(&l402.to_authenticate_string()).unwrap(),
                );
                Ok(res)
            }
        }
        _ => {
            // If the authorization header is missing or does not start with "L402", return a 402 error
            let l402 = L402::new().await; // replace "token" with actual token
            let mut res = StatusCode::PAYMENT_REQUIRED.into_response();
            res.headers_mut().insert(
                "Access-Control-Expose-Headers", HeaderValue::from_str("Www-Authenticate").unwrap());
            res.headers_mut().insert(
                "Www-Authenticate",
                HeaderValue::from_str(&l402.to_authenticate_string()).unwrap(),
            );
            Ok(res)
        }
    }
}

struct L402 {
    token: Macaroon,
    invoice: Option<Bolt11Invoice>,
    preimage: Option<String>,
}

impl L402 {
    async fn new() -> Self {
        let lnaddress = LightningAddress::new(dotenv::var("LNADDRESS").unwrap().as_str()).await;
        let invoice: Bolt11Invoice = lnaddress.get_invoice().await;
        let payment_hash = invoice.payment_hash();
        let token = Self::build_macaroon(payment_hash.to_string());
        L402 {
            token,
            invoice: Some(invoice),
            preimage: None,
        }
    }

    fn to_authenticate_string(&self) -> String {
        format!(
            "L402 token=\"{}\", invoice=\"{}\"",
            self.token.serialize(Format::V2).unwrap(),
            self.invoice.as_ref().unwrap().to_string()
        )
    }

    fn from_auth_header(header: &str) -> crate::Result<Self> {
        let parts: Vec<&str> = header.split(' ').collect();
        if parts.len() != 2 || parts[0] != "L402" {
            return Self::invalid_auth_header_error();
        }

        let macaroon_preimage: Vec<&str> = parts[1].split(':').collect();
        if macaroon_preimage.len() != 2 {
            return Self::invalid_auth_header_error();
        }

        let rune = Macaroon::deserialize(macaroon_preimage[0].to_string()).unwrap();
        let preimage = Some(macaroon_preimage[1].to_string());

        Ok(L402 {
            token: rune,
            invoice: None, // You might want to replace this with actual invoice
            preimage,
        })
    }

    fn is_valid(&self) -> Result<bool, Error> {
        let preimage_hash = Self::get_preimage_hash(self.preimage.as_ref().unwrap());
        let key = Self::get_macaroon_key();
        let mut verifier = Verifier::default();
        verifier.satisfy_exact(
            format!(
                "payment_hash = {}",
                hex::encode(preimage_hash.as_slice()).to_lowercase()
            )
            .as_bytes()
            .into(),
        );
        verifier
            .verify(&self.token, &key, Default::default())
            .map_err(|_| Error::L402Error {
                status: StatusCode::BAD_REQUEST,
                text: "Invalid macaroon".to_string(),
            })?;
        Ok(true)
    }

    fn build_macaroon(payment_hash: String) -> Macaroon {
        let key = Self::get_macaroon_key();
        let mut macaroon = Macaroon::create(Some("location".into()), &key, "id".into()).unwrap();
        macaroon
            .add_first_party_caveat(format!("payment_hash = {}", payment_hash).as_bytes().into());

        macaroon
    }

    fn get_macaroon_key() -> MacaroonKey {
        let secret_hex = env::var("MACAROON_SECRET").expect("MACAROON_SECRET must be set");
        let secret = hex::decode(secret_hex).expect("Failed to decode hex");
        MacaroonKey::generate(&secret)
    }

    fn get_preimage_hash(preimage: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(hex::decode(preimage).unwrap());
        hasher.finalize().to_vec()
    }

    fn invalid_auth_header_error() -> crate::Result<Self> {
        Err(Error::L402Error {
            status: StatusCode::BAD_REQUEST,
            text: "Invalid authorization header".to_string(),
        })
    }
}
