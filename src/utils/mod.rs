// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

use axum::http::uri::PathAndQuery;
use axum::http::{self, HeaderValue, Request};
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

// endregion: --- Modules

// region:    --- Time
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap() // TODO: need to check if safe.
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::DateFailParse(moment.to_string()))
}
// endregion: --- Time

// region:    --- Base64
pub fn b64u_encode(content: &str) -> String {
    base64_url::encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<String> {
    let decoded_string = base64_url::decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::FailToB64uDecode)?;

    Ok(decoded_string)
}
// endregion: --- Base64

// region:    --- Request Manipulation

const HOST: &str = "host";
const X_API_KEY: &str = "x-api-key";
const AUTHORIZATION: &str = "authorization";

pub fn remove_host_header<B>(req: &mut Request<B>) {
    req.headers_mut().remove(HOST);
}

pub fn insert_x_api_key_header<B>(req: &mut Request<B>, auth: &str) {
    req.headers_mut()
        .insert(X_API_KEY, HeaderValue::from_str(auth).unwrap());
}

pub fn insert_auth_bearer_header<B>(req: &mut Request<B>, auth: &str) {
    let auth = format!("Bearer {}", auth);
    req.headers_mut()
        .insert(AUTHORIZATION, HeaderValue::from_str(auth.as_str()).unwrap());
}

pub fn insert_auth_token_header<B>(req: &mut Request<B>, auth: &str) {
    let auth = format!("Token {}", auth);
    req.headers_mut()
        .insert(AUTHORIZATION, HeaderValue::from_str(auth.as_str()).unwrap());
}

pub fn add_key_query_param<B>(req: &mut Request<B>, auth: &str) {
    let mut parts = req.uri().clone().into_parts();
    let key_param = format!("key={}", auth);
    parts.path_and_query = Some(
        PathAndQuery::from_maybe_shared(format!(
            "{}?{}",
            parts
                .path_and_query
                .map(|pq| pq.path().to_string())
                .unwrap_or_default(),
            key_param
        ))
        .unwrap(),
    );
    *req.uri_mut() = http::Uri::from_parts(parts).unwrap();
}
// endregion: --- Request Manipulation
