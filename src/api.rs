use axum::{http::StatusCode, response::IntoResponse};

use crate::api::jwt::AuthError;

mod jwt;
pub mod user;
pub mod counter;

pub enum ApiError{
    Auth(AuthError),
    Internal(anyhow::Error),
}

impl<E> From<E> for ApiError
where 
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> ApiError {
        ApiError::Internal(err.into())
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        ApiError::Auth((err))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}