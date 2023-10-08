use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::str::FromStr;

use macaroon::MacaroonKey;
use once_cell::sync::Lazy;
use reqwest::Url;
use serde_json::Value;
use time::OffsetDateTime;

use super::{get_env, get_env_b64u_as_u8s, get_env_parse, get_env_parse_to_macaroon_key};
use crate::{Error, Result};

static INSTANCE: Lazy<Config> = Lazy::new(|| {
    Config::load_from_env()
        .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
});

pub fn config() -> &'static Config {
    &INSTANCE
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Crypt
    pub PWD_KEY: Vec<u8>,
    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_SEC: f64,
    pub MACAROON_KEY: MacaroonKey,

    // -- Db
    pub DB_URL: String,

    // -- Web
    pub WEB_FOLDER: String,

    // -- Lightning
    pub LIGHTNING_ADDRESS: String,
    pub CASHU_MINT_URL: Url,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- Crypt
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

            TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
            MACAROON_KEY: get_env_parse_to_macaroon_key("SERVICE_MACAROON_KEY")?,

            // -- Db
            DB_URL: get_env("SERVICE_DB_URL")?,

            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,

            // -- Lightning
            LIGHTNING_ADDRESS: get_env("SERVICE_LIGHTNING_ADDRESS")?,
            CASHU_MINT_URL: get_env_parse("SERVICE_CASHU_MINT_URL")?,
        })
    }
}
