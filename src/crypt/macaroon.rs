use macaroon::{Macaroon, MacaroonKey, Verifier};

use super::error::{Error, Result};
use crate::config::config::config;

pub fn generate_macaroon(payment_hash: String) -> Macaroon {
    _generate_macaroon(payment_hash, &config().MACAROON_KEY)
}

pub fn validate_macaroon(macaroon: Macaroon, preimage_hash: Vec<u8>) -> Result<bool> {
    _validate_macaroon(macaroon, preimage_hash, &config().MACAROON_KEY)
}

fn _generate_macaroon(payment_hash: String, key: &MacaroonKey) -> Macaroon {
    let mut macaroon = Macaroon::create(Some("location".into()), &key, "id".into()).unwrap();
    macaroon.add_first_party_caveat(format!("payment_hash = {}", payment_hash).as_bytes().into());

    macaroon
}

fn _validate_macaroon(
    macaroon: Macaroon,
    preimage_hash: Vec<u8>,
    key: &MacaroonKey,
) -> Result<bool> {
    let mut verifier = Verifier::default();
    verifier.satisfy_exact(
        format!(
            "payment_hash = {}",
            hex::encode(preimage_hash).to_lowercase()
        )
        .as_bytes()
        .into(),
    );
    verifier
        .verify(&macaroon, &key, Default::default())
        .map_err(|_| Error::MacaroonCaveatFail)?;
    Ok(true)
}
