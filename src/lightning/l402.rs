use lightning_invoice::Bolt11Invoice;
use macaroon::{Format, Macaroon};
use sha2::Digest;

use super::error::{Error, Result};
use super::LightningAddress;
use crate::config::config::config;
use crate::crypt;

pub struct L402Builder {
    amount: Option<u64>,
    timeout: Option<u64>,
}

impl L402Builder {
    pub fn new() -> Self {
        Self {
            amount: None,
            timeout: None,
        }
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub async fn build(self) -> Result<L402> {
        println!("L402Builder::build");
        let lnaddress = LightningAddress::new(&config().LIGHTNING_ADDRESS).await;
        let invoice_amount: i64 = self.amount.unwrap_or(1000) as i64;
        let invoice: Bolt11Invoice = lnaddress.get_invoice(invoice_amount).await;
        let payment_hash = invoice.payment_hash();
        let timeout = self.timeout.unwrap_or(60 * 60 * 24) as i64;
        let token = crypt::macaroon::generate_macaroon(payment_hash.to_string(), timeout);
        Ok(L402 {
            token,
            invoice: Some(invoice),
            preimage: None,
        })
    }
}

#[derive(Debug)]
pub struct L402 {
    token: Macaroon,
    invoice: Option<Bolt11Invoice>,
    preimage: Option<String>,
}

impl L402 {
    fn new(token: Macaroon, invoice: Option<Bolt11Invoice>, preimage: Option<String>) -> Self {
        Self {
            token,
            invoice,
            preimage,
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

        let token = match Macaroon::deserialize(macaroon_preimage[0].to_string()) {
            Ok(token) => token,
            Err(_) => return Err(Error::L402AuthHeaderInvalidFail),
        };

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
