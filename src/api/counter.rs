use axum::{extract::State, Json};
use sqlx::{Pool, Sqlite};

use crate::db::Counter;

use super::{jwt::Uid, ApiError};

pub async fn list(
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>
) -> Result<Json<Vec<Counter>>, ApiError> {
    todo!();
}