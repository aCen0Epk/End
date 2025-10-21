use axum::{http::StatusCode, response::IntoResponse};

mod jwt;
pub mod user;


pub enum ApiError{
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


impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}