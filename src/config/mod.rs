pub mod apis;
pub mod config;
pub mod replit;

use std::{env, str::FromStr};

use macaroon::MacaroonKey;

use crate::error::{Error, Result};

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

pub fn get_optional_env(name: &'static str) -> Option<String> {
    env::var(name).ok()
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}

pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}

pub fn get_env_parse_to_macaroon_key(name: &'static str) -> Result<MacaroonKey> {
    let key = get_env(name)?;
    let mac_key = MacaroonKey::generate(&key.as_bytes());

    Ok(mac_key)
}
