use macaroon::MacaroonKey;

use crate::{Error, Result};
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
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

    // -- AI API Keys
    pub OPENAI_API_KEY: Option<String>,
    pub PALM_API_KEY: Option<String>,
    pub CLIPDROP_API_KEY: Option<String>,
    pub REPLICATE_API_KEY: Option<String>,
    pub ANTHROPIC_API_KEY: Option<String>,
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

            // -- AI API Keys
            OPENAI_API_KEY: get_optional_env("OPENAI_API_KEY"),
            PALM_API_KEY: get_optional_env("PALM_API_KEY"),
            CLIPDROP_API_KEY: get_optional_env("CLIPDROP_API_KEY"),
            REPLICATE_API_KEY: get_optional_env("REPLICATE_API_KEY"),
            ANTHROPIC_API_KEY: get_optional_env("ANTHROPIC_API_KEY"),
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_optional_env(name: &'static str) -> Option<String> {
    env::var(name).ok()
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_parse_to_macaroon_key(name: &'static str) -> Result<MacaroonKey> {
    let key = get_env(name)?;
    let mac_key = MacaroonKey::generate(&key.as_bytes());

    Ok(mac_key)
}
