use axum::{extract::{Path, State}, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};

use crate::{db::{Counter}};

use super::{jwt::Uid, ApiError};

pub async fn list(
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<Counter>>, ApiError> {
    let counters = sqlx::query_as::<_, Counter>
        ("select * from counters where user_id = ? order by sequence desc",
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await?;
    Ok(Json(counters))
}
#[derive(Debug, Deserialize)]
pub struct AddPayload {
    pub name:String,
    pub value: i32,
    pub step: i32,
    pub input_step: bool,
}

pub async fn add(
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<AddPayload>,
) -> Result<Json<Value>, ApiError> {
    let sequence = sqlx::query_as::<_, (i32,)>(
        "select sequnce from counters where user_id = ? order by sequence desc limit 1"
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await;

    let sequence = match sequence {
        Ok((sequence,)) => sequence + 1,
        Err(sqlx::Error::RowNotFound) => 0,
        Err(e) => return Err(ApiError::from(e)),
    };

    sqlx::query("insert into counters (user_id, name, value, step, input_step, sequence) values (?, ?, ?, ?, ?, ?)")
        .bind(user_id)
        .bind(payload.name)
        .bind(payload.value)
        .bind(payload.step)
        .bind(payload.input_step)
        .bind(sequence)
        .execute(&pool)
        .await?;

    Ok(Json(json!({})))
}

pub async fn show(
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i32>,
) -> Result<Json<Counter>, ApiError> {
    let counter = get_user_counter(id, user_id, &pool).await?;
    Ok(Json(counter))
}

#[derive(Debug, Deserialize)]
pub struct UpdatePayload {
    pub name:String,
    pub step: i32,
    pub input_step: bool,
}


pub async fn update(
    Path(id): Path<i32>,  
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<Value>, ApiError> {
    get_user_counter(id, user_id, &pool).await?;

    sqlx::query("update counters set name = ?, step = ?, updated_at = CURRENT_TIMESTAMP where id = ?")
    .bind(payload.name)
    .bind(payload.step)
    .bind(payload.input_step)
    .bind(id)
    .execute(&pool)
    .await?;

    Ok(Json(json!({})))
}

pub async fn destroy(
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ApiError> {
    get_user_counter(id, user_id, &pool).await?;
    sqlx::query(
        r#"delete from counters where id = ?;
        delete from counter_records where counter_id = ?"#,
    )
    .bind(id)
    .bind(id)
    .execute(&pool)
    .await?;

    Ok(Json(json!({})))
}


pub async fn top(
    Path(id): Path<i32>,
    State(pool): State<Pool<Sqlite>>,
    Uid(user_id):Uid,
) -> Result<Json<Value>, ApiError> {
    get_user_counter(id, user_id, &pool).await?;

    let sequence = sqlx::query_as::<_, (i32,)>(
        "select sequence from counters where user_id = ? order by sequence desc limit 1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await;
    let sequence = match sequence {
        Ok((sequence,)) => sequence + 1,
        Err(e) => return Err(ApiError::from(e)),
    };
    
    sqlx::query("update counters set sequence = ?, updated_at = CURRENT_TIMESTAMP where id = ?")
        .bind(sequence)
        .bind(id)
        .execute(&pool)
    .await?;

    Ok(Json(json!({})))
}

pub  async fn get_user_counter(id: i32, user_id:i32, pool: &Pool<Sqlite>) -> Result<Counter, ApiError> {
    sqlx::query_as::<_, Counter>("select * from counters where id = ? and user_id = ?")
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e|match e {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::from(e),
        })
}