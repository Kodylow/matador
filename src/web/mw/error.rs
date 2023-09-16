use axum::response::{IntoResponse, Response};
use reqwest::{header::InvalidHeaderValue, StatusCode};
use serde::Serialize;
use tracing::debug;

use crate::lightning;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    InvalidHeaderValue(String),
    InvalidRoute(String),
    Lightning(lightning::Error),
}

// region:    --- Froms
impl From<InvalidHeaderValue> for Error {
    fn from(val: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(format!("InvalidHeaderValue: {}", val))
    }
}

impl From<lightning::Error> for Error {
    fn from(val: lightning::Error) -> Self {
        Self::Lightning(val)
    }
}
// endregion: --- Froms

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
