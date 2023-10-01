use std::env;
use std::process::Command;
use std::sync::Mutex;
use std::{collections::HashMap, sync::OnceLock};

use crate::error::Result;
use serde_json::Value;
use time::OffsetDateTime;

use crate::config::config::Config;
use crate::config::get_optional_env;

use super::config::config;

#[derive(Clone, Debug)]
pub struct ApiParams {
    pub key: Option<String>,
    pub host: &'static str,
    pub path: &'static str,
    pub timeout: Option<i64>,
}

impl ApiParams {
    pub fn new(
        key: Option<String>,
        host: &'static str,
        path: &'static str,
        timeout: Option<i64>,
    ) -> Self {
        Self {
            key,
            host,
            path,
            timeout,
        }
    }
}

pub struct ApisConfig {
    pub openai: ApiParams,
    pub clipdrop: ApiParams,
    pub palm: ApiParams,
    pub replicate: ApiParams,
    pub anthropic: ApiParams,
    pub stability: ApiParams,
    pub goose: ApiParams,
    pub cohere: ApiParams,
    pub ai21: ApiParams,
    // pub replit: Mutex<ApiParams>,
}

pub fn apis_config() -> &'static ApisConfig {
    static INSTANCE: OnceLock<ApisConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        ApisConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

impl ApisConfig {
    fn load_from_env() -> Result<ApisConfig> {
        dotenv::dotenv().ok();
        let (replit_key, replit_timeout) = get_optional_replit_key();
        Ok(ApisConfig {
            openai: ApiParams::new(
                get_optional_env("OPENAI_API_KEY"),
                "api.openai.com",
                "/openai",
                None,
            ),
            clipdrop: ApiParams::new(
                get_optional_env("CLIPDROP_API_KEY"),
                "clipdrop-api.co",
                "/clipdrop",
                None,
            ),
            palm: ApiParams::new(
                get_optional_env("PALM_API_KEY"),
                "generativelanguage.googleapis.com",
                "/palm",
                None,
            ),
            replicate: ApiParams::new(
                get_optional_env("REPLICATE_API_KEY"),
                "api.replicate.com",
                "/replicate",
                None,
            ),
            anthropic: ApiParams::new(
                get_optional_env("ANTHROPIC_API_KEY"),
                "api.anthropic.com",
                "/anthropic",
                None,
            ),
            stability: ApiParams::new(
                get_optional_env("STABILITY_API_KEY"),
                "api.stability.ai",
                "/stability",
                None,
            ),
            goose: ApiParams::new(
                get_optional_env("GOOSE_API_KEY"),
                "api.goose.ai",
                "/goose",
                None,
            ),
            cohere: ApiParams::new(
                get_optional_env("COHERE_API_KEY"),
                "api.cohere.ai",
                "/cohere",
                None,
            ),
            ai21: ApiParams::new(
                get_optional_env("AI21_API_KEY"),
                "api.ai21.com",
                "/ai21",
                None,
            ),
            // replit: Mutex::new(ApiParams::new(
            //     replit_key.unwrap().lock().unwrap().clone().into(),
            //     "production-modelfarm.replit.com",
            //     "/replit",
            //     replit_timeout,
            // )),
        })
    }

    pub fn get_params(&self, route: &str) -> Option<ApiParams> {
        match route {
            "openai" => Some(self.openai.clone()),
            "clipdrop" => Some(self.clipdrop.clone()),
            "palm" => Some(self.palm.clone()),
            "replicate" => Some(self.replicate.clone()),
            "anthropic" => Some(self.anthropic.clone()),
            "stability" => Some(self.stability.clone()),
            "goose" => Some(self.goose.clone()),
            "cohere" => Some(self.cohere.clone()),
            "ai21" => Some(self.ai21.clone()),
            // "replit" => Some(self.replit.lock().unwrap().clone()),
            _ => None,
        }
    }

    // pub fn get_replit_key(&self) -> Option<String> {
    //     if self.is_replit_key_expired() {
    //         regenerate_replit_key();
    //     }
    //     self.replit.lock().unwrap().key.clone()
    // }

    // pub fn is_replit_key_expired(&self) -> bool {
    //     self.replit.lock().unwrap().timeout.is_none()
    //         || self.replit.lock().unwrap().timeout.unwrap()
    //             <= OffsetDateTime::now_utc().unix_timestamp()
    // }
}

fn get_optional_replit_key() -> (Option<Mutex<String>>, Option<i64>) {
    // check if in repl
    if !env::var("REPL_ID").is_ok() && !env::var("REPLIT_DEPLOYMENT").is_ok() {
        return (None, None);
    }

    generate_replit_key()
}

// pub fn regenerate_replit_key() {
//     let conf = apis_config();
//     let (new_key, new_timeout) = generate_replit_key();
//     let mut replit = conf.replit.lock().unwrap();
//     replit.key = new_key.unwrap().lock().unwrap().clone().into();
//     replit.timeout = new_timeout;
// }

fn generate_replit_key() -> (Option<Mutex<String>>, Option<i64>) {
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
    let token = v["token"].as_str().map(|s| Mutex::new(s.to_string()));
    let timeout_secs = v["timeout"].as_i64();

    let timeout = timeout_secs.map(|secs| (OffsetDateTime::now_utc().unix_timestamp() + secs));

    println!("Generated Key!");

    (token, timeout)
}
