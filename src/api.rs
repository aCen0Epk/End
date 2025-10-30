use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::api::jwt::AuthError;

mod jwt;
pub mod user;
pub mod counter;
pub mod counter_record;

pub enum ApiError{
    NotFound,
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
        ApiError::Auth(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND).into_response(),
            ApiError::Auth(err) => err.into_response(),
            ApiError::Internal(err) => {
                let body = Json(json!({
                    "error":err.to_string(),
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            },
        }
    }
}