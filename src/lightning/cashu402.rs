use lightning_invoice::Bolt11Invoice;
use macaroon::{Format, Macaroon};
use reqwest::Url;
use sha2::Digest;

use super::error::{Error, Result};
use super::LightningAddress;
use crate::config::config::config;
use crate::crypt;

pub struct Cashu402Builder {
    amount: Option<u64>,
    timeout: Option<u64>,
}

impl Cashu402Builder {
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

    pub async fn build(self) -> Result<Cashu402> {
        let mint_url = config().CASHU_MINT_URL.clone();
        let amount: u64 = self.amount.unwrap_or(3) as u64;
        Ok(Cashu402 { mint_url, amount })
    }
}

#[derive(Debug)]
pub struct Cashu402 {
    mint_url: Url,
    amount: u64,
}

impl Cashu402 {
    fn new(mint_url: Url, amount: u64) -> Self {
        Self { mint_url, amount }
    }

    pub fn to_authenticate_string(&self) -> String {
        format!(
            "Cashu402 mint_url=\"{}\", amount=\"{}\"",
            self.mint_url, self.amount
        )
    }
}
