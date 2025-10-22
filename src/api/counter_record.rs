use axum::{extract::{Path, State}, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{Pool, Sqlite};

use crate::{api::{counter::{self, get_user_counter}, user}, db::{Counter, CounterRecord, User}};

use super::{jwt::Uid, ApiError};


#[derive(Debug, Deserialize)]
pub struct AddPayload {
    counter_id: i32,
    pub step: i32,
}


pub async fn add(
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<AddPayload>,
) -> Result<Json<Value>, ApiError> {
    let counter = get_user_counter(payload.counter_id, user_id, &pool)
    .await?;

    let next_value = counter.value + payload.step;

    sqlx::query(
        r#"insert into counter_records (counter_id, step, begin, end) values (?, ?, ?, ?);
        update counters set value = ?, update_at = CURRENT_TIMESTAMP where id = ?"#,
    )
        .bind(payload.counter_id)
        .bind(payload.step)
        .bind(counter.value)
        .bind(next_value)
        .bind(next_value)
        .bind(payload.counter_id)
        .execute(&pool)
        .await?;

    Ok(Json(json!({})))
}

pub async fn list(
    Path(counter_id): Path<i32>,
    Uid(user_id): Uid,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<CounterRecord>>, ApiError> {
    get_user_counter(counter_id, user_id, &pool) .await?;


    let records  = sqlx::query_as::<_, CounterRecord>
        ("select * from counter_records where user_id = ? order by id desc",
    )
    .bind(counter_id)
    .fetch_all(&pool)
    .await?;
    Ok(Json(records))
}