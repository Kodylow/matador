pub mod middleware_auth;
pub mod routes_openai;
use once_cell::sync::Lazy;
use std::env;

// read the env vars in and set OPENAI_API_KEY

pub static OPENAI_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"));
