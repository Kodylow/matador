use axum::{
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    // OpenAI Errors
    OpenAIError { status: StatusCode, text: String },
}

// Note: never ever pass through a server error to the client
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:12} - Error::into_response", "HANDLER");

        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the error into the response
        response.extensions_mut().insert(self);

        response
    }
}
