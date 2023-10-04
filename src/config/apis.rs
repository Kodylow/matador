use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::sync::Mutex;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use serde_json::Value;
use time::OffsetDateTime;
use tracing::{debug, info};

use super::replit::{get_optional_replit, ReplitApiParams};
use crate::config::config::Config;
use crate::config::get_optional_env;
use crate::error::Result;

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

    pub fn set_key(&mut self, key: String) {
        self.key = key;
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

    pub fn is_expired(&self) -> bool {
        match self.timeout {
            Some(timeout) => {
                let now = OffsetDateTime::now_utc().unix_timestamp();
                now > timeout
            }
            None => false,
        }
    }
}

#[derive(Debug)]
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
    pub scenario: Option<ApiParams>,
    pub perplexity: Option<ApiParams>,
    pub anyscale: Option<ApiParams>,
    pub replit: Option<ReplitApiParams>,
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
            "scenario" => self.scenario.clone(),
            "perplexity" => self.perplexity.clone(),
            "anyscale" => self.anyscale.clone(),
            _ => None,
        }
    }
}

#[derive(Debug)]
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
    scenario: Option<ApiParams>,
    perplexity: Option<ApiParams>,
    anyscale: Option<ApiParams>,
    replit: Option<ReplitApiParams>,
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
            scenario: None,
            perplexity: None,
            anyscale: None,
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

    pub fn scenario(mut self) -> Self {
        if let Some(key) = get_optional_env("SCENARIO_API_KEY") {
            self.scenario = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.cloud.scenario.gg")
                    .path("/scenario")
                    .timeout(None),
            );
        }

        self
    }

    pub fn perplexity(mut self) -> Self {
        if let Some(key) = get_optional_env("PERPLEXITY_API_KEY") {
            self.perplexity = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.perplexity.ai")
                    .path("/perplexity")
                    .timeout(None),
            );
        }

        self
    }

    pub fn anyscale(mut self) -> Self {
        if let Some(key) = get_optional_env("ANYSCALE_API_KEY") {
            self.anyscale = Some(
                ApiParams::new()
                    .key(key)
                    .host("api.endpoints.anyscale.com")
                    .path("/anyscale")
                    .timeout(None),
            );
        }

        self
    }

    pub fn replit(mut self) -> Self {
        if let Some(replit) = get_optional_replit() {
            self.replit = Some(replit);
        }

        self
    }

    pub fn build(self) -> ApisConfig {
        info!("Building ApisConfig");
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
            scenario: self.scenario,
            perplexity: self.perplexity,
            anyscale: self.anyscale,
            replit: self.replit,
        }
    }
}

pub static APIS_CONFIG: Lazy<ApisConfig> = Lazy::new(|| {
    dotenv::dotenv().ok();
    ApisConfigBuilder::new()
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
        .scenario()
        .perplexity()
        .anyscale()
        .replit()
        .build()
});

pub fn apis_config() -> &'static ApisConfig {
    &APIS_CONFIG
}
