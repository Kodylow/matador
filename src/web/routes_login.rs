use axum::{routing, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web, Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", routing::post(api_login))
}

// You can only have 1 `body extractor` per handler, and it has to be the last parameter
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login - {payload:?}", "HANDLER");

    if payload.username != "admin" || payload.pwd != "admin" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
