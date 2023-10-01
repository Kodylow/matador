use macaroon::MacaroonKey;

use crate::{Error, Result};
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;
use std::process::Command;
use time::OffsetDateTime;
use serde_json::Value;

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

    // -- Replit Dynamic API Key
    pub REPLIT_API_KEY: Option<String>,
    pub REPLIT_API_KEY_TIMEOUT: Option<u64>,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        let (replit_key, replit_timeout) = get_optional_replit_key();
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

            // -- Replit Dynamic API Key
            REPLIT_API_KEY: replit_key,
            REPLIT_API_KEY_TIMEOUT: replit_timeout,
        })
    }

    fn get_replit_key(&mut self) -> Option<String> {
        // Get the current UNIX timestamp using the `time` crate
        let now = OffsetDateTime::now_utc().unix_timestamp();

        // Check if the key is expired
        if self.REPLIT_API_KEY_TIMEOUT.is_none() || self.REPLIT_API_KEY_TIMEOUT.unwrap() as i64 <= now {
            let (new_key, new_timeout) = generate_replit_key();
            self.REPLIT_API_KEY = new_key;
            self.REPLIT_API_KEY_TIMEOUT = new_timeout;
        }
        
        self.REPLIT_API_KEY.clone()
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
    
fn get_optional_replit_key() -> (Option<String>, Option<u64>) {
    // check if in repl
    if !env::var("REPL_ID").is_ok() && !env::var("REPLIT_DEPLOYMENT").is_ok() {
        return (None, None);
    }
        
    generate_replit_key()
}

fn generate_replit_key() -> (Option<String>, Option<u64>) {
    println!("Replit Dynamic API Key ...");
    let repl_slug = env::var("REPL_SLUG").expect("REPL_SLUG not set");
    let script_path = format!("/home/runner/{}/replit/get_token.py", repl_slug);

    let proc = Command::new("python")
        .arg(script_path)
        .output()
        .expect("Failed to execute Get Replit API KEY process");
    let proc_stdout = String::from_utf8_lossy(&proc.stdout);

    if proc_stdout.is_empty() {
        return (None, None);
    }

    let proc_stdout = proc_stdout.trim();

    // Parse the output into a JSON Value
    let v: Value = serde_json::from_str(proc_stdout).expect("Failed to parse JSON");
    
    // Extract the token and timeout from the JSON Value
    let token = v["token"].as_str().map(|s| s.to_string());
    let timeout_secs = v["timeout"].as_u64();

    let timeout = timeout_secs.map(|secs| (OffsetDateTime::now_utc().unix_timestamp() + secs as i64) as u64);

    println!("Generated Key!");

    (token, timeout)
}