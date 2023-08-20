use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    // Auth Errors
    LoginFail,
    AuthFailNoAuthTokenCookie,
    AuthFailInvalidToken,
    AuthFailCtxNotInExt,
    // Model Errors
    TicketDeleteFailIdNotFound { id: u64 },
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

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- Auth
            Self::AuthFailCtxNotInExt
            | Self::AuthFailInvalidToken
            | Self::AuthFailNoAuthTokenCookie => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // -- Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
