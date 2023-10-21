pub mod apis;
pub mod config;
pub mod replit;

use anyhow::{anyhow, Error, Result};
use macaroon::MacaroonKey;
use std::env;
use std::str::FromStr;

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|e| anyhow!("{}: {}", name, e))
}

pub fn get_optional_env(name: &'static str) -> Option<String> {
    env::var(name).ok()
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>()
        .map_err(|e| anyhow!("{}, couldn't parse env variable", name))
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|e| anyhow!("{}: {}", name, e))
}

pub fn get_env_parse_to_macaroon_key(name: &'static str) -> Result<MacaroonKey> {
    let key = get_env(name)?;
    let mac_key = MacaroonKey::generate(&key.as_bytes());

    Ok(mac_key)
}
