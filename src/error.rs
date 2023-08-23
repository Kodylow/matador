use axum::{
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    // Router Errors
    RouterError { text: String },
    // OpenAI Errors
    OpenAIError { status: StatusCode, text: String },
    // ClipDrop Errors
    ClipDropError { status: StatusCode, text: String },
    // MakerSuite Errors
    MakerSuiteError { status: StatusCode, text: String },

    // Lightning Errors
    L402Error { status: StatusCode, text: String },
}

// Note: never ever pass through a server error to the client
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("Error: {:?}", self);

        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the error into the response
        response.extensions_mut().insert(self);

        response
    }
}
