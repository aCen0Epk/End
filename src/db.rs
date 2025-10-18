use std::env;

use sqlx::{Pool, Sqlite, SqlitePool};
use time::PrimitiveDateTime;


pub async fn establish_connection() -> Pool<Sqlite> {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("can't connect to database");
    pool
}


#[derive(sqlx::FromRow)]
pub struct  User {
    pub id: i32,
    pub openid: String,
    pub session_key: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}