use std::{env, process::Command};

use serde_json::Value;
use time::OffsetDateTime;

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
            let new_params = generate_replit_key();
            self.params = new_params;
        }

        self.params.key.clone()
    }
}

pub fn get_optional_replit() -> Option<ReplitApiParams> {
    // check if in repl
    if !env::var("REPL_ID").is_ok() && !env::var("REPLIT_DEPLOYMENT").is_ok() {
        return None;
    }
    Some(ReplitApiParams {
        params: generate_replit_key(),
    })
}

pub fn generate_replit_key() -> ApiParams {
    println!("Replit Dynamic API Key ...");
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

    // Parse the output into a JSON Value
    let v: Value = serde_json::from_str(proc_stdout).expect("Failed to parse JSON");

    // Extract the token and timeout from the JSON Value
    let token = v["token"].to_string();
    let timeout_secs = v["timeout"].as_i64();

    let timeout = timeout_secs.map(|secs| (OffsetDateTime::now_utc().unix_timestamp() + secs));

    println!("Generated Key!");

    ApiParams::new()
        .key(token)
        .host("production-modelfarm.replit.com")
        .path("/replit")
        .timeout(timeout)
}
