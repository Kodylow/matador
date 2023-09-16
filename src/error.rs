use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::debug;

use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    // -- Router
    RouterFailToSetRoutes(&'static str),

    // -- Modules
    Model(model::Error),
}

// region:    --- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
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
