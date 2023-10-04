use std::env;
use std::process::Command;

use serde::Deserialize;
use time::OffsetDateTime;
use tracing::info;

use super::apis::ApiParams;

#[derive(Clone, Debug)]
pub struct ReplitApiParams {
    pub params: ApiParams,
}

impl ReplitApiParams {
    pub fn new() -> Self {
        Self {
            params: ApiParams::new(),
        }
    }

    pub fn get_key(&mut self) -> String {
        if self.params.is_expired() {
            info!("Replit API key has expired, generating a new one and updating the config");
            let new_params = generate_replit_key();
            self.params = new_params;
        } else {
            info!("Replit API key is still valid...");
        }

        self.params.key.clone()
    }
}

pub fn get_optional_replit() -> Option<ReplitApiParams> {
    // check if in repl
    if !env::var("REPL_ID").is_ok() && !env::var("REPLIT_DEPLOYMENT").is_ok() {
        print!("Not in repl. Skipping replit api...");
        return None;
    }

    info!("In repl, getting replit api key...");
    Some(ReplitApiParams {
        params: generate_replit_key(),
    })
}

#[derive(Deserialize)]
pub struct ReplitTokenManagerResponse {
    pub token: String,
    pub timeout: i64,
}

pub fn generate_replit_key() -> ApiParams {
    info!("Replit Dynamic API Key ...");
    let repl_slug = env::var("REPL_SLUG").expect("REPL_SLUG not set");
    let script_path = format!("/home/runner/{}/replit/get_token.py", repl_slug);

    let proc = Command::new("python")
        .arg(script_path)
        .output()
        .expect("Failed to execute Get Replit API KEY process");
    let proc_stdout = String::from_utf8_lossy(&proc.stdout);

    if proc_stdout.is_empty() {
        return ApiParams::new();
    }

    let proc_stdout = proc_stdout.trim();

    // Parse the output into the ReplitTokenManagerResponse struct
    let res: ReplitTokenManagerResponse =
        serde_json::from_str(&proc_stdout).expect("Failed to parse JSON");

    info!("Generated Key!");

    ApiParams::new()
        .key(res.token)
        .host("production-modelfarm.replit.com")
        .path("/replit")
        .timeout(Some(res.timeout))
}
