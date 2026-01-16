use sqlx::{Pool, Postgres};

use crate::models::Board;

pub async fn all(pool: Pool<Postgres>) -> Result<Vec<Board>, sqlx::Error> {
    sqlx::query_as!(Board, "SELECT * FROM boards ORDER BY name")
        .fetch_all(&pool)
        .await
}

pub async fn get(pool: Pool<Postgres>, board_name: &str) -> Result<Board, sqlx::Error> {
    sqlx::query_as!(Board, "SELECT * FROM boards WHERE name = $1", board_name)
        .fetch_one(&pool)
        .await
}
