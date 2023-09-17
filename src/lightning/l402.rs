use crate::crypt;

use super::error::{Error, Result};
use super::LightningAddress;

use lightning_invoice::Bolt11Invoice;
use macaroon::{Format, Macaroon};
use sha2::Digest;

pub struct L402 {
    token: Macaroon,
    invoice: Option<Bolt11Invoice>,
    preimage: Option<String>,
}

impl L402 {
    pub async fn new() -> Self {
        let lnaddress = LightningAddress::new(dotenv::var("LNADDRESS").unwrap().as_str()).await;
        let invoice: Bolt11Invoice = lnaddress.get_invoice(1000).await;
        let payment_hash = invoice.payment_hash();
        let token = crypt::macaroon::generate_macaroon(payment_hash.to_string());
        L402 {
            token,
            invoice: Some(invoice),
            preimage: None,
        }
    }

    pub fn is_valid(&self) -> Result<bool> {
        let preimage_hash = get_preimage_hash(self.preimage.as_ref().unwrap());
        Ok(crypt::macaroon::validate_macaroon(
            self.token.clone(),
            preimage_hash,
        )?)
    }

    pub fn to_authenticate_string(&self) -> String {
        format!(
            "L402 token=\"{}\", invoice=\"{}\"",
            self.token.serialize(Format::V2).unwrap(),
            self.invoice.as_ref().unwrap().to_string()
        )
    }

    pub fn from_auth_header(header: &str) -> Result<Self> {
        let parts: Vec<&str> = header.split(' ').collect();
        if parts.len() != 2 || parts[0] != "L402" {
            return Err(Error::L402AuthHeaderInvalidFail);
        }

        let macaroon_preimage: Vec<&str> = parts[1].split(':').collect();
        if macaroon_preimage.len() != 2 {
            return Err(Error::L402AuthHeaderInvalidFail);
        }

        let token = Macaroon::deserialize(macaroon_preimage[0].to_string()).unwrap();
        let preimage = Some(macaroon_preimage[1].to_string());

        Ok(L402 {
            token: token,
            invoice: None,
            preimage,
        })
    }
}

fn get_preimage_hash(preimage: &str) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(hex::decode(preimage).unwrap());
    hasher.finalize().to_vec()
}
