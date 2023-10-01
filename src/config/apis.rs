use std::env;
use std::process::Command;
use std::sync::Mutex;
use std::{collections::HashMap, sync::OnceLock};

use crate::error::Result;
use serde_json::Value;
use time::OffsetDateTime;

use crate::config::config::Config;
use crate::config::get_optional_env;

#[derive(Clone, Debug)]
pub struct ApiParams {
    pub key: String,
    pub host: &'static str,
    pub path: &'static str,
    pub timeout: Option<i64>,
}

impl ApiParams {
    pub fn new() -> Self {
        Self {
            key: "".to_string(),
            host: "",
            path: "",
            timeout: None,
        }
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = key;
        self
    }

    pub fn host(mut self, host: &'static str) -> Self {
        self.host = host;
        self
    }

    pub fn path(mut self, path: &'static str) -> Self {
        self.path = path;
        self
    }

    pub fn timeout(mut self, timeout: Option<i64>) -> Self {
        self.timeout = timeout;
        self
    }
}

pub struct ApisConfig {
    pub openai: Option<ApiParams>,
    pub clipdrop: Option<ApiParams>,
    pub palm: Option<ApiParams>,
    pub replicate: Option<ApiParams>,
    pub anthropic: Option<ApiParams>,
    pub stability: Option<ApiParams>,
    pub goose: Option<ApiParams>,
    pub cohere: Option<ApiParams>,
    pub ai21: Option<ApiParams>,
    pub together: Option<ApiParams>,
    pub replit: Option<Mutex<ApiParams>>,
}

impl ApisConfig {
    pub fn get_params(&self, route: &str) -> Option<ApiParams> {
        match route {
            "openai" => self.openai.clone(),
            "clipdrop" => self.clipdrop.clone(),
            "palm" => self.palm.clone(),
            "replicate" => self.replicate.clone(),
            "anthropic" => self.anthropic.clone(),
            "stability" => self.stability.clone(),
            "goose" => self.goose.clone(),
            "cohere" => self.cohere.clone(),
            "ai21" => self.ai21.clone(),
            "together" => self.together.clone(),
            // "replit" => self.replit.lock().unwrap().clone()),
            _ => None,
        }
    }
}

pub struct ApisConfigBuilder {
    openai: Option<ApiParams>,
    clipdrop: Option<ApiParams>,
    palm: Option<ApiParams>,
    replicate: Option<ApiParams>,
    anthropic: Option<ApiParams>,
    stability: Option<ApiParams>,
    goose: Option<ApiParams>,
    cohere: Option<ApiParams>,
    ai21: Option<ApiParams>,
    together: Option<ApiParams>,
    replit: Option<ApiParams>,
}

impl ApisConfigBuilder {
    pub fn new() -> Self {
        Self {
            openai: None,
            clipdrop: None,
            palm: None,
            replicate: None,
            anthropic: None,
            stability: None,
            goose: None,
            cohere: None,
            ai21: None,
            together: None,
            replit: None,
        }
    }

    pub fn openai(mut self) -> Self {
        if let Some(key) = get_optional_env("OPENAI_API_KEY") {
            self.openai = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.openai.com")
                    .path("/openai")
                    .timeout(None),
            );
        }
        self
    }

    pub fn clipdrop(mut self) -> Self {
        if let Some(key) = get_optional_env("CLIPDROP_API_KEY") {
            self.clipdrop = Some(
                ApiParams::new()
                    .key(key)
                    .host("clipdrop-api.co")
                    .path("/clipdrop")
                    .timeout(None),
            );
        }
        self
    }

    pub fn palm(mut self) -> Self {
        if let Some(key) = get_optional_env("PALM_API_KEY") {
            self.palm = Some(
                ApiParams::new()
                    .key(key)
                    .host("generativelanguage.googleapis.com")
                    .path("/palm")
                    .timeout(None),
            );
        }
        self
    }

    pub fn replicate(mut self) -> Self {
        if let Some(key) = get_optional_env("REPLICATE_API_KEY") {
            self.replicate = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.replicate.com")
                    .path("/replicate")
                    .timeout(None),
            );
        }
        self
    }

    pub fn anthropic(mut self) -> Self {
        if let Some(key) = get_optional_env("ANTHROPIC_API_KEY") {
            self.anthropic = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.anthropic.com")
                    .path("/anthropic")
                    .timeout(None),
            );
        }
        self
    }

    pub fn stability(mut self) -> Self {
        if let Some(key) = get_optional_env("STABILITY_API_KEY") {
            self.stability = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.stability.ai")
                    .path("/stability")
                    .timeout(None),
            );
        }
        self
    }

    pub fn goose(mut self) -> Self {
        if let Some(key) = get_optional_env("GOOSE_API_KEY") {
            self.goose = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.goose.ai")
                    .path("/goose")
                    .timeout(None),
            );
        }
        self
    }

    pub fn cohere(mut self) -> Self {
        if let Some(key) = get_optional_env("COHERE_API_KEY") {
            self.cohere = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.cohere.ai")
                    .path("/cohere")
                    .timeout(None),
            );
        }
        self
    }

    pub fn ai21(mut self) -> Self {
        if let Some(key) = get_optional_env("AI21_API_KEY") {
            self.ai21 = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.ai21.com")
                    .path("/ai21")
                    .timeout(None),
            );
        }
        self
    }

    pub fn together(mut self) -> Self {
        let key = get_optional_env("TOGETHER_API_KEY");
        if let Some(key) = get_optional_env("TOGETHER_API_KEY") {
            self.together = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.together.xyz")
                    .path("/together")
                    .timeout(None),
            );
        }
        self
    }

    pub fn replit(mut self) -> Self {
        let (replit_key, replit_timeout) = get_optional_replit_key();
        self.replit = Some(
            ApiParams::new()
                .key(replit_key.unwrap().lock().unwrap().clone().into())
                .host("production-modelfarm.replit.com")
                .path("/replit")
                .timeout(replit_timeout),
        );
        self
    }

    pub fn build(self) -> ApisConfig {
        ApisConfig {
            openai: self.openai,
            clipdrop: self.clipdrop,
            palm: self.palm,
            replicate: self.replicate,
            anthropic: self.anthropic,
            stability: self.stability,
            goose: self.goose,
            cohere: self.cohere,
            ai21: self.ai21,
            together: self.together,
            replit: self.replit.map(|p| Mutex::new(p)),
        }
    }
}

pub fn apis_config() -> &'static ApisConfig {
    static INSTANCE: OnceLock<ApisConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        dotenv::dotenv().ok();
        return ApisConfigBuilder::new()
            .openai()
            .clipdrop()
            .palm()
            .replicate()
            .anthropic()
            .stability()
            .goose()
            .cohere()
            .ai21()
            .together()
            // .replit()
            .build();
    })
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
